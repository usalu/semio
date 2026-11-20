# Composition prepare fixes (2026-11-18)

1. stdio `composition prepare`: `📚️idml/📦️packages/🦀️rust/Cargo.toml` lacked `[package.metadata.semio.composition] manifest = "../../🧩️composition/🔣️.json"` (the contribution JSON already existed). Added, same as docx/md. A follow-on failure (`composition apps repeat order`, stdio-bim) came from `💬️bcf/🧩️composition/🔣️.json` reusing app orders 10/11 of ifc; bcf is now 12/13. Result: contributions=38 apps=180, `🤖️generated/🧩️composition/🦀️.rs` exists.
2. remodeling `composition prepare`: `🎥️video/📦️packages/🦀️rust/Cargo.toml` had avi/mp4 as hard deps and no `# 🧩️ Capability Features` / `Capability Dependencies` regions. Added both regions plus `default = ["video-containers"]` (source uses avi/mp4 unconditionally); generator then emitted optional deps and features. Result: targets=2.

Note: `native-launch.ts` joins cwd to the repo root, so pass a repo-relative cwd, not absolute.
