const fs = require('fs');
const path = '🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs';
const before = fs.readFileSync(path, 'utf8');
const old = 'let row_copy=original_return_claim_copy_bytes();if cx.retained_grant().maximum_copy_bytes<row_copy{return Ok(false)}let epoch=cx.original_source_epoch().unwrap_or(0);';
const next = 'let row_copy=original_return_claim_copy_bytes();let current_copy=cx.retained_grant().maximum_copy_bytes;let epoch=cx.original_source_epoch().unwrap_or(0);';
const marker = 'if fuel<3{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original return claim exceeds its remaining lifetime work currency").into())}let Some(credit)';
const replacement = 'if fuel<3{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original return claim exceeds its remaining lifetime work currency").into())}if current_copy<row_copy{return Ok(false)}let Some(credit)';
if (before.split(old).length !== 2 || before.split(marker).length !== 2) throw Error('expected exact preflight');
const after = before.replace(old, next).replace(marker, replacement);
const tmp = path + '.guest-return-edit';
fs.writeFileSync(tmp, after, { flag: 'wx' });
if (fs.readFileSync(path, 'utf8') !== before) { fs.unlinkSync(tmp); throw Error('concurrent source change'); }
fs.renameSync(tmp, path);
