#!/usr/bin/env python3
"""🔗️ R9 item 4: removes the plugin SDK's weak-linkage bundle-installer shim (an implicit second initialization
path) and its dead `no_mangle` install exports. Every guest export already calls the embedding crate's explicit
runtime ensure first (`__semio_actor_exports!`/`__semio_owned_core_exports!` → `$ensure()`), so the shim either
re-assembled the plugin bundle a second time (plugins) or never ran (extensions). The wasm panic report moves into
that explicit ensure so it is installed before the bundle is assembled.

Usage: `python3 plugin-link-shim-removal.py [--apply]` (default: dry run, exits 1 if any hunk does not match)."""
import sys

ROOT = "/Users/ueli/Documents/semio/"
SDK = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
CRATE_ROOT = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🦀️.rs"
IO = ROOT + "🧰️framework/🔨️modules/🚪️io/🦀️.rs"
DEMONSTRATOR = ROOT + "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/🦀️.rs"

HUNKS = {
    SDK: [
        ("""    static PLUGIN_INIT_ONCE: std::sync::Once = std::sync::Once::new();

    // 🚫️async: E4 fn-pointer slot
    static PLUGIN_BUNDLE_INSTALLER: std::sync::OnceLock<fn()> = std::sync::OnceLock::new();

    /// @emoji 🧩️ Registers the embedding plugin crate's bundle installer (expanded from `plugin_exports!`).
    pub fn register_plugin_bundle_installer(install: fn()) {
        let _ = PLUGIN_BUNDLE_INSTALLER.set(install);
    }

    /// 🔗️ Weak default so intermediate `cdylib` links (e.g. `semio-framework-os` pulled into a
    /// wasip2 plugin build via feature unification of `component-guest`) succeed; the embedding
    /// plugin's `plugin_exports!` provides the strong installer override.
    #[cfg(feature = "component-guest")]
    #[unsafe(no_mangle)]
    #[linkage = "weak"]
    pub extern "C" fn semio_plugin_bundle_installer_link_shim() {}

    /// Ensures the embedding plugin crate's bundle installer ran before any WIT export is served.
    // 🚫️async: E1 pure `std::sync::Once` init consumed by `component::wasip2`'s sync `world
    // actor` `Guest` impls (WIT-fixed, no `host-async` import exists to await in that world) — R9.
    // Body is entirely `call_once`'s sync closure (an `E4` fn-pointer install), zero suspension.
    pub fn ensure_plugin_initialized() {
        PLUGIN_INIT_ONCE.call_once(|| {
            // 🩹️ The owned hook uses only `std`, so every browser actor reports the exact Rust panic
            // through its WASI stderr stream before the target's normal abort trap terminates it.
            #[cfg(target_arch = "wasm32")]
            std::panic::set_hook(Box::new(|panic| eprintln!("[DEBUG] [semio-plugin panic] {panic}")));
            // 🧬️ A2 (design-abi.md §4): `host_port`/`HostBackboneChannel`/`set_host_backbone_channel`
            // are deleted per `important.md`'s "Replace, never wrap" list — a process-global backbone
            // channel cannot survive a pooled multi-instance actor. The per-instance `EffectBackbone`
            // that replaces it is NOT implemented in this wave (the VCS `BackboneChannelPort` trait's
            // `send`/`poll` methods are synchronous; bridging them onto the async `Effect`/`Event`
            // request-response model needs its own design decision — see the report's `lease-request`/
            // deferred-work section). No backbone channel is registered here; VCS backbone-dependent
            // paths surface a real "no host backbone linked" error instead of silently no-op'ing.
            #[cfg(feature = "component-guest")]
            {
                semio_plugin_bundle_installer_link_shim();
                if let Some(install) = PLUGIN_BUNDLE_INSTALLER.get() {
                    install();
                }
            }
        });
    }
""", """    /// 🩹️ Reports every guest panic through the WASI stderr stream before the target's abort trap ends
    /// the instance. The embedding crate's runtime ensure (`plugin_exports!`/`extension_exports!`)
    /// calls it once, before its bundle is assembled, so a panic during assembly is reported too.
    // 🚫️async: E1 called from the sync `std::sync::Once` closure of the macro-generated runtime ensure.
    pub fn install_guest_panic_report() {
        #[cfg(target_arch = "wasm32")]
        std::panic::set_hook(Box::new(|panic| eprintln!("[semio-plugin panic] {panic}")));
    }
"""),
        ("""            fn __semio_ensure_plugin_runtime() {
                static ONCE: std::sync::Once = std::sync::Once::new();
                ONCE.call_once(__semio_install_plugin_bundle);
            }
""", """            fn __semio_ensure_plugin_runtime() {
                static ONCE: std::sync::Once = std::sync::Once::new();
                ONCE.call_once(|| {
                    $crate::plugin_runtime::install_guest_panic_report();
                    __semio_install_plugin_bundle();
                });
            }
"""),
        ("""            // 🚫️async: E4 fn-pointer slot — this fn's VALUE is stored in `PLUGIN_BUNDLE_INSTALLER:
            // OnceLock<fn()>` via `register_plugin_bundle_installer`, so it can never itself be
            // `async fn`; the pure bundle install remains directly callable from this fn-pointer slot.
            fn __semio_install_plugin_bundle() {
                __SEMIO_PLUGIN_RUNTIME.with(|runtime| {
                    $crate::plugin_runtime::install_plugin_bundle_result(runtime, ($bundle_fn)());
                });
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_plugin_bundle_installer_link_shim() {
                $crate::plugin_runtime::register_plugin_bundle_installer(__semio_install_plugin_bundle);
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_plugin_install_bundle() {
                __semio_install_plugin_bundle();
            }
""", """            // 🚫️async: E1 called from the sync `std::sync::Once` closure of `__semio_ensure_plugin_runtime`.
            fn __semio_install_plugin_bundle() {
                __SEMIO_PLUGIN_RUNTIME.with(|runtime| {
                    $crate::plugin_runtime::install_plugin_bundle_result(runtime, ($bundle_fn)());
                });
            }
"""),
        ("""            // without wasm in the loop — natively installs the bundle (bypassing
            // `ensure_plugin_initialized`'s `component-guest`-gated weak-linkage shim, which only
            // fires on a real wasm build) and byte-compares `describe::describe_plugin()`'s packed
""", """            // without wasm in the loop — natively installs the bundle and byte-compares
            // `describe::describe_plugin()`'s packed
"""),
        ("""    // 🚫️async: E4 fn-pointer slot
    static EXTENSION_BUNDLE_INSTALLER: std::sync::OnceLock<fn()> = std::sync::OnceLock::new();

    /// 🧩️ Registers the embedding extension crate's bundle installer (expanded from `extension_exports!`).
    pub async fn register_extension_bundle_installer(install: fn()) {
        let _ = EXTENSION_BUNDLE_INSTALLER.set(install);
    }

    /// 🔗️ Weak default mirroring `semio_plugin_bundle_installer_link_shim` (see that symbol's own
    /// doc comment): lets an intermediate link succeed before the embedding extension crate's
    /// `extension_exports!` provides the strong override. Without this default AND the explicit call
    /// below, `EXTENSION_BUNDLE_INSTALLER` is never populated — no code path ever invoked this
    /// symbol, so `register_extension_bundle_installer` was silently never called and every real
    /// extension's `manifest()`/`activate()` observed only the empty-default `ExtensionBundle`.
    #[cfg(feature = "component-extension-guest")]
    #[unsafe(no_mangle)]
    #[linkage = "weak"]
    pub extern "C" fn semio_extension_bundle_installer_link_shim() {}

    /// Ensures the embedding extension crate's bundle installer ran before any WIT export is served
    /// — mirrors `ensure_plugin_initialized`'s explicit weak/strong-linkage shim call.
    async fn ensure_extension_initialized() {
        EXTENSION_BUNDLE.with(|slot| {
            if slot.borrow().is_none() {
                #[cfg(feature = "component-extension-guest")]
                unsafe {
                    semio_extension_bundle_installer_link_shim();
                }
                if let Some(install) = EXTENSION_BUNDLE_INSTALLER.get() {
                    install();
                }
            }
        });
    }

""", ""),
        ("""            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_extension_bundle_installer_link_shim() {
                $crate::app::resolve_ready($crate::plugin_runtime::register_extension_bundle_installer(__semio_install_plugin_bundle));
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_extension_install_bundle() {
                __semio_install_plugin_bundle();
            }
        };""", """        };"""),
        ("""                ONCE.call_once(|| {
                    __semio_install_extension_bundle();
                    $crate::app::resolve_ready($crate::plugin_runtime::extension_activate()).expect("installed extension activation");
                });""", """                ONCE.call_once(|| {
                    $crate::plugin_runtime::install_guest_panic_report();
                    __semio_install_extension_bundle();
                    $crate::app::resolve_ready($crate::plugin_runtime::extension_activate()).expect("installed extension activation");
                });"""),
        ("""            // 🚫️async: E4 fn-pointer slot — see `plugin_exports!`'s `__semio_install_plugin_bundle` doc.
            fn __semio_install_extension_bundle() {
                $crate::app::resolve_ready($crate::plugin_runtime::install_extension_bundle(($bundle_fn)()));
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_extension_bundle_installer_link_shim() {
                $crate::app::resolve_ready($crate::plugin_runtime::register_extension_bundle_installer(__semio_install_extension_bundle));
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_extension_install_bundle() {
                __semio_install_extension_bundle();
            }
""", """            // 🚫️async: E1 called from the sync `std::sync::Once` closure of `__semio_ensure_extension_runtime`.
            fn __semio_install_extension_bundle() {
                $crate::app::resolve_ready($crate::plugin_runtime::install_extension_bundle(($bundle_fn)()));
            }
"""),
    ],
    CRATE_ROOT: [
        ("""#![cfg_attr(any(feature = "component-guest", feature = "component-extension-guest"), feature(linkage))]
""", ""),
    ],
    IO: [
        ("""should reach it (host boot / guest `ensure_plugin_initialized`).""", """should reach it (host boot / the guest runtime ensure `plugin_exports!` generates)."""),
    ],
    DEMONSTRATOR: [
        ("""// IS the single terminal wasm component (nothing depends on it as a lib), so it always owns the
// `semio_plugin_install_bundle` entry point — no other crate ever needs to disable it. This""",
         """// IS the single terminal wasm component (nothing depends on it as a lib), so it always owns the
// `plugin_exports!` entry points — no other crate ever needs to disable them. This"""),
    ],
}

REPEATED = {
    SDK: [("\n        ensure_plugin_initialized();\n", "\n", 5), ("\n        ensure_extension_initialized().await;\n", "\n", 3)],
}


def main() -> int:
    apply = "--apply" in sys.argv
    failed = False
    for path in sorted(set(HUNKS) | set(REPEATED)):
        text = open(path, encoding="utf-8").read()
        for old, new in HUNKS.get(path, []):
            count = text.count(old)
            print(f"{'ok ' if count == 1 else 'BAD'} {count} {path.rsplit('/', 3)[-1]} :: {old.strip().splitlines()[0][:90]}")
            failed |= count != 1
            text = text.replace(old, new, 1)
        for old, new, expected in REPEATED.get(path, []):
            count = text.count(old)
            print(f"{'ok ' if count == expected else 'BAD'} {count}/{expected} {path.rsplit('/', 3)[-1]} :: {old.strip()}")
            failed |= count != expected
            text = text.replace(old, new)
        if apply and not failed:
            open(path, "w", encoding="utf-8").write(text)
    for path in (SDK, CRATE_ROOT):
        residue = [token for token in ("linkage", "link_shim", "BUNDLE_INSTALLER", "ensure_plugin_initialized", "ensure_extension_initialized", "install_bundle()") if token in (open(path, encoding="utf-8").read() if apply and not failed else "")]
        if residue:
            print("RESIDUE", path, residue)
            failed = True
    print("applied" if apply and not failed else "dry run clean" if not failed else "FAILED")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
