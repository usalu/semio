"""🧰 Stage the verified raw picker owner in the SDK's real media declaration crate."""

import hashlib
import json
import os
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUTPUT = TICKET / "🗑️generated"
PLUGIN = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
MEDIA = PLUGIN / "📜️declaration/🎞️media"
MANIFEST = MEDIA / "📦️packages/🦀️rust/Cargo.toml"
RAW = MEDIA / "📥️original-media"
LIFETIME = PLUGIN / "⚛️reactor/🚪️lifetime/🦀️.rs"
INGRESS_TURN = PLUGIN / "⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs"

DEPENDENCIES = {
    "semio-framework-replication-command-ingress": Path("🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/📦️packages/🦀️rust"),
    "semio-framework-value-persistent": Path("🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/📦️packages/🦀️rust"),
}
MODULES = {
    "ingress": ("🎟️loan.rs", "📜️original-media-ingress.rs"),
    "receiver": ("📥️receiver.rs", "📜️original-media-paged-receiver.rs"),
    "bridge": ("🌉️fixed-page.rs", "📜️original-media-command-bridge.rs"),
    "command_owner": ("📥️command-owner.rs", "📜️original-media-command-owner.rs"),
    "guest": ("🛂️guest.rs", "📜️original-media-guest-reloan.rs"),
    "raw_picker": ("🧾️picker.rs", "raw-picker-receipts.rs"),
    "raw_picker_pack": ("📦️capsule.rs", "raw-picker-pack.rs"),
}
PUBLIC = """pub use command_owner::OriginalMediaFormatInput;
pub use guest::{OriginalGuestMediaInput, OriginalMediaPickerIntent};
pub use ingress::{OriginalMediaLoan, ORIGINAL_MEDIA_PAGE_BYTES};
pub use raw_picker::{Ack, Control, FormatReceipt, FormatResult, Phase, RawPicker, ResultCode, Transport};
pub use raw_picker_pack::{DecodedLoan, picker_args};
pub const RAW_MEDIA_BYTES: usize = semio_framework_replication_command_ingress::COMMAND_MAXIMUM_BYTES;
pub type RawMediaTarget = [u64; 8];
pub type RawMediaPicker = RawPicker<RawMediaTarget, RAW_MEDIA_BYTES>;
pub type RawMediaInput = OriginalGuestMediaInput<RawMediaTarget, RAW_MEDIA_BYTES>;
pub type RawMediaIntent = OriginalMediaPickerIntent<RawMediaTarget>;
"""
ADMIT_SOURCE = """    /// 🪪 Admits one command Source only under the exact current native allocation and lifetime.
    pub fn admit_command_origin(
        &mut self, key: NativeCloseKey, cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if !self.cell.is_live() || self.cell.owner().map(NativeLifetimeOwner::key) != Some(key) {
            return Ok(false);
        }
        self.command_origin.admit_step(cx)
    }

"""


def replace(source, old, new):
    if source.count(old) != 1:
        raise ValueError(f"Raw media owner source anchor drifted: {old}")
    return source.replace(old, new)


def stage():
    changes = {}
    preimages = {}

    def guarded(path, edit):
        source = (ROOT / path).read_bytes()
        preimages[str(path)] = hashlib.sha256(source).hexdigest()
        changes[str(path)] = edit(source.decode())

    def add(path, source):
        if (ROOT / path).exists() or len(str(ROOT / path)) > 256:
            raise ValueError(f"Raw media source cannot be staged: {path}")
        preimages[str(path)] = None
        changes[str(path)] = source

    for dependency, path in DEPENDENCIES.items():
        if f'name = "{dependency}"' not in (ROOT / path / "Cargo.toml").read_text(encoding="utf8"):
            raise ValueError(f"Raw media dependency edge drifted: {dependency}")
    edges = "".join(f'{name} = {{ path = "{Path(os.path.relpath(ROOT / location, ROOT / MANIFEST.parent)).as_posix()}" }}\n'
                    for name, location in DEPENDENCIES.items())
    guarded(MANIFEST, lambda text: replace(text, "semio-framework-value = { workspace = true }\n",
                                            "semio-framework-value = { workspace = true }\n" + edges))

    def lock_dependencies(text):
        header = 'name = "semio-plugin-decl-media"'
        start = text.index(header)
        end = text.index("[[package]]", start)
        stanza = text[start:end]
        for anchor, name in zip(("semio-framework-job-payload", "semio-framework-value"), DEPENDENCIES):
            line = f' "{anchor}",\n'
            stanza = replace(stanza, line, line + f' "{name}",\n')
        return text[:start] + stanza + text[end:]

    guarded(Path("Cargo.lock"), lock_dependencies)
    media_module = '#[path = "📥️original-media/🦀️.rs"]\npub mod original_media;\n'
    guarded(MEDIA / "🦀️.rs", lambda text: replace(text, '#[path="⚠️turn/🦀️.rs"]\n',
                                                  media_module + '#[path="⚠️turn/🦀️.rs"]\n'))
    guarded(PLUGIN / "🦀️.rs", lambda text: replace(text, 'pub mod structured_media_export;\n',
                                                    'pub mod structured_media_export;\npub use semio_plugin_decl_media::original_media;\n'))
    lifetime_impl = "impl<PA:semio_plugin_reactor_app::PluginApp> NativeLifetimeSlot<PA>{\n"
    guarded(LIFETIME, lambda text: replace(text, lifetime_impl, lifetime_impl + ADMIT_SOURCE))
    original_source = '        let admitted={let mut lifetimes=runtime.guest_lifetimes.try_borrow_mut().map_err(|_|reactor_close_fault("command Source lifetime is busy"))?;let slot=lifetimes.get_mut(cursor.instance).ok_or_else(||reactor_close_fault("command Source lifetime is missing"))?;slot.command_origin.admit_step(cx)?};'
    authenticated = """        let key = native_close_key(runtime, cursor.instance)?;
        let admitted = {
            let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("command Source lifetime is busy"))?;
            let slot = lifetimes.get_mut(cursor.instance).ok_or_else(|| reactor_close_fault("command Source lifetime is missing"))?;
            slot.admit_command_origin(key, cx)?
        };"""
    guarded(INGRESS_TURN, lambda text: replace(text, original_source, authenticated))

    root = '//! 📥️ Receives a picked original file under the exact mounted command Source.\n'
    for module, (target, filename) in MODULES.items():
        source = (TICKET / filename).read_text(encoding="utf8")
        if module == "ingress":
            start = source.index("pub const ORIGINAL_MEDIA_PAGE_BYTES")
            end = source.index("#[derive(Clone, Copy)]", start)
            first = source.index("pub(crate) fn media_type_valid")
            last = source.index("/// 🪟", first)
            source = '//! 🧾️ Defines the original picker loan and fixed page extent.\n\n' + source[start:end] + source[first:last]
        else:
            source = source.replace("use crate::", "use super::")
        root += f'#[path = "{target}"]\npub(crate) mod {module};\n'
        add(RAW / target, source)
    add(RAW / "🦀️.rs", root + "\n" + PUBLIC)
    for path, expected in preimages.items():
        current = ROOT / path
        actual = hashlib.sha256(current.read_bytes()).hexdigest() if current.is_file() else None
        if actual != expected:
            raise ValueError(f"Raw media product preimage changed: {path}")

    OUTPUT.mkdir(exist_ok=True)
    (OUTPUT / "raw-sdk-overlay.json").write_text(json.dumps({"preimages": preimages, "candidate": changes}, ensure_ascii=False), encoding="utf8")
    linked = OUTPUT / "rm-sdk"
    linked.mkdir(exist_ok=True)
    for path, source in changes.items():
        current = Path(path)
        if current.parent == RAW:
            (linked / current.name).write_text(source, encoding="utf8")
    print(f"[DEBUG] Guarded {len(changes)} SDK raw media product postimages")


if __name__ == "__main__":
    stage()
