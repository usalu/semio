# UI Shell Find Dialog Zero-Consumer Audit

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- ShellFindDialog component SHA-256: `a01c4faad61a86a0264d40f2e9efddf37edc8cb0acda01235238e6d949247619`, clean.
- Exclusive story SHA-256: `1001151e87833bfb5bef41d5d051451905166b05437977113efa160ed55b0f3e`, clean.
- ShellSearchDialog documentation referrer SHA-256: `2eb01a0aee48ae564952e1cfe4d34ca00988244c600dc3e079ae08da93391f19`, clean.
- React index at audit time: `50c0bcd05afc285101da820bb3fcae8dd0d8cf8046e64cacdf9dcfce1c6b859f`.

No active production component imports or renders ShellFindDialog. Its closure contains only the implementation, exclusive story, mechanical barrel, one exclusive UI package smoke test, and a ShellSearchDialog docstring that describes its shared result row. Tests/stories/glue/docs do not qualify as production consumers.

Decision: delete the zero-consumer component/story, remove its barrel/test, and update the shared-row docstring to name only the remaining ShellSearchDialog owner. Do not create a module, wrapper, alias, replacement, or compatibility export.
