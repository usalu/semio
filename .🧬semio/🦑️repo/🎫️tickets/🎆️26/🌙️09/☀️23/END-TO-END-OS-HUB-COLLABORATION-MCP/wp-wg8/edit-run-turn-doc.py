import pathlib
path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
start = text.index("        /// 🎠️ terra-kernel-loop: the real loop the packet brief's item 1 asks for")
end = text.index("        /// ⏯️ One request's turn for `actor`, settled the way the React host settles one")
terra = text[start:end]
assert terra.count("///") > 10 and "fn " not in terra
text = text[:start] + text[end:]
once_old = '''        /// 🎯️ One granted turn, answering what the guest still owes: `None` once it settled, else the
        /// events of the continuation turn (see [`Self::run_turn`]).
'''
assert text.count(once_old) == 1
once_new = terra + '''        ///
        /// 🎯️ One granted turn, answering what the guest still owes: `None` once it settled, else the
        /// events of the continuation turn (see [`Self::run_turn`]). A granted actor the shard reports
        /// [`ShardOutcome::Preempted`] — ours or one `Kernel::tick` granted beside it — is resumed right
        /// here with an `Event::Wake` envelope (the shard's "resume with no events"), so the tick loop
        /// keeps granting it until its turn returns; `deadline` bounds how long that may take.
'''
text = text.replace(once_old, once_new)
path.write_text(text)
print("doc moved")
