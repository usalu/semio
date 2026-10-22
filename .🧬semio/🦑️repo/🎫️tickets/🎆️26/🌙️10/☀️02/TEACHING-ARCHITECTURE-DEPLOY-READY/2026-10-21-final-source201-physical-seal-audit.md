# Final Source201 Physical Seal Audit

Cohort `9966a1ade363c4d787cf0916fffc9bb0`: physical audit clear at the final epoch scan. Exactly302 unique leaves comprise101 control/input leaves and201 Source leaves. Both complete passes matched all expected byte counts and SHA-256 pins; zero mismatch, zero cross-pass physical drift, zero final named/ancestry epoch drift. This is physical evidence only; semantic clearances remain scoped to their separate reports.

Each read opened O_RDONLY|O_NOFOLLOW|O_NONBLOCK, required an ordinary single-link file, used a65536-byte buffer and refused content exceeding its expected pin. Full descriptor and named dev/ino/mode/nlink/size/mtimeNs/ctimeNs epochs matched before/after. Every ancestor was an ordinary directory, with dev/ino/mode identities stable throughout both passes and final scan. Final scan rechecked every named leaf and its ancestry after both full stream passes.

The101 controls are the83 fresh combined inputs,6 prefix owners,8 guest owners including guest own-input seal, combined-input seal, prefix-input seal, and2 identical authored/generated intents. Source rows came exclusively from the fresh current201 Source map. No old168/195 map was treated as current.

## Exact Final Seal Pins

- Guest own inputs995 / `e57822ca4583c542309fad0e40d52c604b1413721b2198a542225a743f4ce065`.
- Prefix core14728 / `cc3bb1d2c43b1b7d41fa458f2ab3cf3eecf2dd8e7a178c14faafe2599491b707`.
- Combined inputs66646 / `f49dd8e76d9b28d3da4db93ace40cccaf431be3d3714e0d8a011540f2f9531cd`.
- Prefix inputs14224 / `e09aa23b55979ff4666ce96fae2b4a89152175e32e53cb45d4ef854f24374bfc`.
- Both intent files8829 / `f19742dc1f115b60e91da8b4cb1287c16b37dec817eaf676f0501474765ef55d`.

## Audit Digests

Content rows (repository-relative path, kind, bytes, sha256), sorted by path.localeCompare and JSON.stringify: `2e5b6e3096d3ff20221c21a8f58f0f15a2d9925512febca56dd92eb08c6fd7c2`.

Physical rows (repository-relative path, full leaf epoch, absolute ancestor identities), identically sorted and serialized: `c198b28cf115f6d2fd3bf23a616d73abcd1f1a7f21ce1f097ae57a3433b4562c`.

No Native invocation, runtime Source/test evaluation, log reservation, Source/control edit or shared ledger write occurred. Audit Markdown is the sole authored output. This does not claim execution or product build success.