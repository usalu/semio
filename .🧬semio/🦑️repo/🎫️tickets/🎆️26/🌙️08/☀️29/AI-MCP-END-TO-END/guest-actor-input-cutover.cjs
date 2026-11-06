const fs = require('node:fs');
const path = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🔁️persistent-local/🦀️.rs';
const before = fs.readFileSync(path, 'utf8');
let after = before;
function replace(from, to) {
  if (after.split(from).length !== 2) throw new Error(`expected one occurrence: ${from}`);
  after = after.replace(from, to);
}
replace('#[cfg(all(target_arch = "wasm32", target_env = "p2"))]\n#[doc(hidden)]\npub mod owned_abi {', '#[doc(hidden)]\npub mod owned_abi {');
replace('    #[derive(Deserialize, Serialize)]\n    pub struct PollInput {', '    #[derive(Deserialize, Serialize, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]\n    pub struct PollInput {');
replace('    #[derive(Deserialize, FromValue, Serialize, ToValue)]\n    pub struct RestoreInput {', '    #[derive(Deserialize, FromValue, Serialize, ToValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]\n    pub struct RestoreInput {');
replace('    #[derive(Deserialize, FromValue, Serialize, ToValue)]\n    pub struct CodecInput {', '    #[derive(Deserialize, FromValue, Serialize, ToValue, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]\n    pub struct CodecInput {');
const marker = '    /// 📦️ Output of the owned `codec.genesis`/`codec.apply-ops`/`codec.replay-envelopes` core exports.';
replace(marker, '    /// 📥️ One explicit actor input owns its original typed payload across the borrowed wire admission.\n    #[derive(Deserialize, Serialize, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned)]\n    pub enum ActorInput {\n        Job(semio_framework::kernel::OriginalJobSource),\n        Poll(PollInput),\n        Codec(CodecInput),\n        Restore(RestoreInput),\n    }\n\n' + marker);
const temporary = '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/guest-input-schema.tmp';
fs.mkdirSync(require('node:path').dirname(temporary), {recursive:true});
fs.writeFileSync(temporary, after);
if (fs.readFileSync(path, 'utf8') !== before) throw new Error('shared schema changed; refusing publication');
fs.renameSync(temporary, path);
