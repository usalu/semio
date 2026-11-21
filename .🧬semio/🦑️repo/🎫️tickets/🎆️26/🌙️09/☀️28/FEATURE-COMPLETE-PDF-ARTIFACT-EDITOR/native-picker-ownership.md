# Native Picker Ownership

The native WGPU shell retains one interactive PNG picker. While it is open, a distinct guest-issued loan goes through the existing original-media worker with an empty selection. The worker sends an extent-zero Cancel and waits for the matching acknowledgment. The frame pump drains the resulting effects under the refused request's mounted controller and removes the worker when it settles. A repeated copy of an outstanding loan shares the existing task.

The focused executable holds one document's dialog open, repeats its loan, and presents another document's loan. The second loan is cancelled with its exact identity while the first picker stays open. Its returned effect reaches the second controller. After the first dialog settles, a fresh second-document loan opens a picker.

The executable uses the live shell task logic, live original-media transport, native Kernel and Value libraries, and Tokio's bounded mailbox. Its dialog and actor response are controlled fixtures.

- [Mounted fixture](native-picker-ownership.json)
- [Rust law](native-picker-ownership-law.rs)
- [Executable checker](native-picker-ownership-check.py)
