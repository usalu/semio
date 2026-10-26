# Minimal Font Test Environment

The seven glyph tests pass in the separate pinned project environment `🗑️generated/s`, with no skips, in 10.907 seconds. The owned command finished with exit status 0.

## Reproduction

Run the ticket-local `minimal-font-test.ts` through the existing repo launcher. It invokes `uv run --project C:/git/semio/🎓️teaching --python 3.12 --group test --frozen python -c …`, loading only `test_manim_fonts.py`. `UV_PROJECT_ENVIRONMENT` selects `🗑️generated/s`; `UV_CACHE_DIR` selects `🗑️generated/uv`. Test output, font cache, TEMP and TMP all select `🗑️generated/minimal-font-test` or its children. `SEMIO_FONTTOOLS_TEST_PATH` and `PYTHONPATH` are removed from the child environment.

## Evidence

uv installed the declared test-only FontTools 4.63.0 into the project environment. The test process reported Manim 0.22.0, the `generated/s/Scripts/python.exe` interpreter and matching environment prefix. Its initial Python import path contains only that environment and the managed Python 3.12 standard library; it contains neither `generated/v` nor the repository `.venv`. The FontTools override was null. The independent FontTools cmap comparison ran successfully rather than skipping. The other six tests cover all 44 declared symbols, the known Georgia gaps, German text and supported math geometry/cache stability, affected cache invalidation, explicit ranges, and the math wrapper.

Captured command output is under `🗑️generated/minimal-font-test/test.log`. Upstream pydub emitted Python syntax warnings for its regular-expression escapes; these did not affect the tests.

The frozen `manim_fonts.py` SHA256 remains `00275cb1bfb43493a3dfbb8612226fcd475120d6f61e2f7f400944f296c553a2`. No product files, authored assets, helper code, runtime tests, course renders or live processes were changed. All owned diagnostic processes have ended.
