# Plan: Get Python Tests Running Again

## Problem Analysis

The pytest discovery fails with:
```
ImportError: libEGL.so.1: cannot open shared object file: No such file or directory
```

This happens because `py/engine/engine.py` unconditionally imports PySide6 at the module level (lines 66-68):
```python
import PySide6.QtCore
import PySide6.QtGui
import PySide6.QtWidgets
```

PySide6 requires libEGL.so.1, which is not available in the headless dev container environment.

However, PySide6 is only used for system tray functionality at lines 1621-1675 - it's not needed for the tests at all.

## Solution

Make the PySide6 imports lazy/conditional by:
1. Removing the top-level PySide6 imports
2. Importing PySide6 only inside the functions that actually need it (the system tray/GUI code)

This allows:
- Tests to run without PySide6/libEGL dependencies
- The GUI functionality to still work when libEGL is available

## Implementation Steps

1. Remove the top-level PySide6 imports from `engine.py` (lines 66-68)
2. Add lazy imports inside the functions that use PySide6 (around line 1621+)
3. Run pytest to verify tests pass

## Files to Modify

- `py/engine/engine.py`
