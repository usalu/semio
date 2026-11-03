# R2 SK report: skip audit (2026-11-01)

Ruling applied (`r2-coordinator-rulings.md`): a skip is allowed only for a missing external tool that `repo` does not provision. Bun is never a skip reason.

Provisioning check (`repo tools list`): `ffmpeg` is repo-registered (`system`, currently `missing` on this host). `qpdf`, `rsvg-convert` and `unzip` are not registered, so they are not provisioned.

## Inventory and decisions

| Location | Skip | Decision |
|---|---|---|
| `🧪️tests/🎞️frame-timing/🟦️.ts` (`describe.skipIf(!encoders)`) | ffmpeg/ffprobe | Invalid: ffmpeg is repo-registered. Fixed: `beforeAll` throws when the encoders are missing. |
| `🧪️tests/🌡️catalog-physics/🟦️.ts:31` (`if (!body) return;`) | silent conditional pass | Invalid: a silent pass. Fixed: `it.each(ids.filter(... thermal.body))`, the guard removed, `body!`. |
| `🧪️tests/🎭️cast-export/🟦️.ts:348` (`it.skipIf(process.platform === "win32")`, SIGTERM exit 130) | platform | Invalid under the ruling. Not fixed (see Open). This is the single skip cv2 saw. |
| `🧪️tests/📕️pdf-reopen/🟦️.ts:11` (`hasTool("qpdf")`) | qpdf | Valid: not provisioned. |
| `🧪️tests/🗜️zip-reopen/🟦️.ts:13` (`hasTool("rsvg-convert")`) | rsvg-convert | Valid: not provisioned. |
| `🧪️tests/📐️paper-dimensions/🟦️.ts:32` (`it.skipIf(!hasRsvg)`) | rsvg-convert | Valid: not provisioned. |
| `🎯️targets/⚛️react/🧪️tests/🧪️playground-compose/🟦️.tsx:64` (`it.skipIf(!hasRsvg)`) | rsvg-convert | Valid: not provisioned. |
| `V/🧪️tests/🐍️integration.py:127` and `:162` (`self.skipTest("Native Windows font-map regression")`) | platform (`sys.platform != "win32"`) | Invalid under the ruling. Not fixed (see Open). |

Clip tests and `🎨️visuals/🧪️tests`: no `@skip*`, `unittest.SkipTest` or `pytest.skip` markers found beyond the two `skipTest` calls above. The `skip_animations` and `skip_folders` hits are not skips.

## Edits

1. `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎞️frame-timing/🟦️.ts`: `beforeAll` imported. The describe is no longer `skipIf`. It throws `ffmpeg and ffprobe are required` when `encoders` is false.
2. `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🌡️catalog-physics/🟦️.ts`: the body check runs over `ids.filter((id) => catalogAsset(id).thermal.body)`. The silent `return` is removed.

## Test output (ticket alias config `q2-vitest-alias.config.ts`, files catalog-physics, frame-timing, cast-export)

- Test Files: 1 failed, 2 passed (3)
- Tests: 403 passed, 4 skipped (407)
- Failed suite: `frame-timing` > encoded clips. `ffmpeg and ffprobe are required`. Expected on this host: `ffmpeg` is registered but `missing`, so the suite now fails loudly.
- Skipped: 3 frame-timing tests (children of the failed `beforeAll`) and 1 cast-export SIGTERM test (win32).

## Open

- Owner decision needed: `cast-export` SIGTERM (`process.platform === "win32"`) and the two `integration.py` native-Windows tests (`sys.platform != "win32"`). These are platform skips, which the ruling does not allow. On Windows, Node cannot deliver SIGTERM with exit 130, and the Win32 memory test cannot run on Linux or macOS. A minimal Edit does not fix either. Options: a cross-platform cancel signal in the CLI, or a platform-specific test file that the platform runner selects, not a skip.
- `ffmpeg` is repo-registered but `missing` here. Provisioning it (or `repo tools setup`) will turn the frame-timing suite green.
- The `vitest` run used ticket-local alias config, so `repo dependencies ensure` was not required.
- Not run: the Python unittest runner (no Python file edited).

## Follow-up: frame-timing skip instead of fail (agent fs)

Ruling applied (`r2-coordinator-rulings.md`, "Platform tests"): a `system` tool that `repo` registers but cannot provision may skip with a visible reason.

- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎞️frame-timing/🟦️.ts`: the `beforeAll` throw is removed (and its import). The encoded-clip block is now `describe.skipIf(!encoders)`. Its title carries the reason when skipped: `🎬️ encoded clips, skipped: ffmpeg (system tool, set SEMIO_TOOL_FFMPEG) not found`. `encoders` still uses `hasTool("ffmpeg")`, the same `repo tools exec` lookup that honours `SEMIO_TOOL_FFMPEG` and PATH. The catalog-physics fix is untouched.
- Known imprecision: when ffmpeg is present but ffprobe is missing, the reason still names only ffmpeg.
- Vitest (ticket alias config `q2-vitest-alias.config.ts`, filter `frame-timing`): Test Files 1 passed (1). Tests 2 passed | 3 skipped (5). Exit 0. The encoded-clip suite is reported as skipped, not failed. ffmpeg is still missing on this host, so the encoded-clip cases did not run here. They run when ffmpeg is present.
