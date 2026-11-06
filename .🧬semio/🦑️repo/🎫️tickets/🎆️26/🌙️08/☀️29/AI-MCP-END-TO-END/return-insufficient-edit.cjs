const fs = require('fs');
const path = '🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs';
const before = fs.readFileSync(path, 'utf8');
const old = 'if available.maximum_items<3||available.maximum_copy_bytes<copy_bytes.checked_add(extra).and_then(|copy|copy.checked_add(row_copy)).ok_or_else(||refused("return claim copy overflow"))?||fuel<3{return Ok(false)}';
const next = 'if available.maximum_items<3||available.maximum_depth==0||available.maximum_copy_bytes<copy_bytes.checked_add(extra).and_then(|copy|copy.checked_add(row_copy)).ok_or_else(||refused("return claim copy overflow"))?{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original return claim exceeds its remaining lifetime ownership currency").into())}if fuel<3{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original return claim exceeds its remaining lifetime work currency").into())}';
if (before.split(old).length !== 2) throw Error('expected exactly one original preflight');
const after = before.replace(old, next);
const tmp = path + '.guest-return-edit';
fs.writeFileSync(tmp, after, { flag: 'wx' });
if (fs.readFileSync(path, 'utf8') !== before) { fs.unlinkSync(tmp); throw Error('concurrent source change'); }
fs.renameSync(tmp, path);
