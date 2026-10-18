# Windows Ordinary Mapped Snapshot

The private Fixed reader now supports a single current physical snapshot with a mapped-root observation while the original host, backing and receipt descriptors remain held. The ordinary dispatch and public Windows publication guard are unchanged.

`captureFixedWindowsMappedV1` consumes the existing canonical Native metadata mapping codec. It pins the exact `fixed.vhd` path, a positive filesystem extent within the current declared policy, and the canonical `m` root/GUID pair. It invokes `captureFixedWindowsCurrentV1` once. The optional branch opens one additional native mapped-root descriptor and compares complete native facts around its actual settle boundary, requiring a directory, no reparse flag, exact root path, exact GUID final path, and a volume distinct from the host/backing. Existing final named-file, host roster and ancestor fences still run after the mapped observation.

The mapping is DATA, not evidence that a particular disk/partition/device is actually associated with the declared backing. The future fixed command owner must independently observe and bind the complete canonical device association before using this snapshot. The returned root facts do not authorize payload reads, allocation, publication, readiness or deletion.

The new schema-owned observation is readonly and uses the existing actual ordinary receiver/preparation account. One additional simultaneous root handle fits the already-derived eight-extra-handle extent; no account reset, descriptor ceiling or policy increase was introduced. The default no-mapping branch performs no mapped-root read, and its prior implementation reconstructs exactly by removing only the new optional branch/type/import members and private factory.

## Source Evidence

The first executable failure after repairing a test template quoting error was semantic: the actual mapped factory was missing. The Source test extracts both actual defining functions; no alternate production reader is tested instead. Closed current-capture vectors increased from 22 to 39. The actual System root codec and actual Native metadata codec are used, with explicit Native syscall/handle Source ports. Ajv validates the portable fixture and SQLite independently classifies the admitted cases.

The mapped ports assert that all three original descriptors remain open. Negative vectors cover wrong image, host-volume confusion, oversized/zero extent, unknown mapping fields, file/reparse root, wrong root GUID/final path, serial/full128 identity drift, actual cancellation, exact undefined/frozen failure, and close failure. The older Python full128 arithmetic oracle remains part of the six-law suite.

- Initial semantic RED: `windows-ordinary-mapped-red2.log`, missing actual mapped factory.
- First Source GREEN: `windows-ordinary-mapped-green.log`, six laws, 363 assertions, natural exit 0.
- First strict failure: `windows-ordinary-mapped-owner-strict.log`, three new unknown-field type diagnostics; explicit primitive narrowing corrected the private codec result.
- Final strict: `windows-ordinary-mapped-owner-strict2.log`, four roots, natural exit 0 and no diagnostics.
- Current-byte Source: `windows-ordinary-mapped-final-source.log`, six laws/363 assertions, natural exit 0.
- Prior actual same-Original Root0 regression: `windows-ordinary-mapped-owner-source.log`, 32 workers, one law/506 assertions, natural exit 0 (15.20 seconds). The genuine Generated FD and SQLite guard cases remain substantive; no Native or Windows hardware capability is synthesized.
- Final manifest: `Generated/bazel-planner/windows-ordinary-mapped-seven-manifest.json`, seven current inputs. The inverse receipt `windows-ordinary-mapped-fixed-inverse.json` matches the previous full Fixed bytes/SHA; the final conditional check edit remains wholly within that same removable branch.

## Remaining Boundary

No Windows Native filesystem or installer was executed. This does not release ordinary consumers, persistent creation, disposable/funded retirement or PUB Windows phase-context integration. Those must use the actual owner/account and immediate native fences coherently before the current public guard can be removed. Actual NTFS/Windows hardware qualification remains separate.
