"""probe.py apply|revert <key> : one-line behaviour-neutral edit of a dashboard crate (backup under 🗑️generated/rust-g/probe-<key>.orig). Keys: labels (cap fn of the terminal labels crate), cli, daemon."""
import hashlib, os, shutil, sys
ROOT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/"
G = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-g/"
EDITS = {
    "prefs": (ROOT + "⚙️preferences/🦀️.rs", "fn invalid(message: impl Into<String>) -> io::Error { io::Error::new(io::ErrorKind::InvalidData, message.into()) }", "fn invalid(message: impl Into<String>) -> io::Error { let _rust_g: u8 = 1; io::Error::new(io::ErrorKind::InvalidData, message.into()) }"),
    "labels": (ROOT + "🖥️terminal/🌐️labels/🦀️.rs", "fn cap(budget: usize) -> u16 { u16::try_from(budget).unwrap_or(u16::MAX) }", "fn cap(budget: usize) -> u16 { let _rust_g: u8 = 1; u16::try_from(budget).unwrap_or(u16::MAX) }"),
}
action, key = sys.argv[1], sys.argv[2]
path, old, new = EDITS[key]
backup = G + "probe-" + key + ".orig"
text = open(path, encoding="utf8").read()
if action == "apply":
    if old not in text or new in text:
        sys.exit("probe anchor not found exactly once or already applied")
    os.makedirs(G, exist_ok=True)
    shutil.copyfile(path, backup)
    open(path, "w", encoding="utf8").write(text.replace(old, new, 1))
    print("applied", key, hashlib.sha256(text.encode()).hexdigest()[:12])
else:
    if new not in text:
        print("nothing to revert"); sys.exit(0)
    open(path, "w", encoding="utf8").write(text.replace(new, old, 1))
    print("reverted", key)
