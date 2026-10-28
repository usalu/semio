# Inspection Duration Probe

The current global pass stopped during chapter cursor propagation, before a final presentation could be published. The outer exception in speech.probe_duration was a TypeError because process.stderr was None. The preceding subprocess reader-thread traceback identifies the actual cause: Windows CP1252 could not decode byte 0x8F in FFmpeg's UTF-8 diagnostic path containing repository emoji.

The existing probe uses subprocess.run(capture_output=True, text=True), whose default text decoder follows the Python interpreter's UTF-8 mode or locale. PYTHONIOENCODING controls the interpreter's standard streams; it does not make that subprocess decoder UTF-8. The normal authoring TypeScript entry already supplies PYTHONUTF8=1, and ToolSession supplies UTF-8 environment values to its own children. The task-specific supervisor must also enable UTF-8 on the actual global compositor Python process.

The narrow recovery is an explicit `-X utf8` before the actual render entry in the saved supervisor COMMAND and before the staged publisher entry. Enabling it only on an outer supervisor does not propagate the interpreter flag to a separately launched Python compositor. This is portable across the supported Python hosts and leaves every source/settings certificate intact. No shared speech/runtime/font/visual source change is necessary, and no source certificate may be rewritten to conceal an authored change.

The immutable original traceback remains in generated/i/master.log or its archived supervisor history. Successful real chapter propagation and independent final decode remain the acceptance evidence after the launcher correction.
