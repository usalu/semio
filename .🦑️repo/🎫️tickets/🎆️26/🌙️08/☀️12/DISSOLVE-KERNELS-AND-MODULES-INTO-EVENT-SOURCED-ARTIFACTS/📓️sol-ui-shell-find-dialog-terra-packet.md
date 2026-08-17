# Terra Packet UI-ShellFindDialog-01: Zero-Consumer Dissolution

- Read AGENTS/audit; apply_patch only, no modifying Git.
- Require component SHA `a01c4faad61a86a0264d40f2e9efddf37edc8cb0acda01235238e6d949247619`, story SHA `1001151e87833bfb5bef41d5d051451905166b05437977113efa160ed55b0f3e`, and ShellSearchDialog SHA `2eb01a0aee48ae564952e1cfe4d34ca00988244c600dc3e079ae08da93391f19`.
- Current shared index hash is announced by coordinator; never edit it.

Terra owns ShellFindDialog component/story, the exact ShellSearchDialog docstring referrer, and unique acceptance `📓️terra-ui-shell-find-dialog-zero-active-consumer-dissolution-acceptance.md`. Delete component/story and remove only the obsolete ShellFindDialog name/link from the shared-row docstring while retaining its ShellSearchDialog responsibility. Checkpoint and wait for coordinator barrel/test registrar.

After registrar signal, run active stale identifier/path/JSX scans, scoped ordinary/cached diff checks, and UI React lint/typecheck/test-quick/build once. Do not touch other UI behavior, shared index, manifests/locks, generated census, Storybook config, protected renderer, or plugins; do not repair unrelated failures.
