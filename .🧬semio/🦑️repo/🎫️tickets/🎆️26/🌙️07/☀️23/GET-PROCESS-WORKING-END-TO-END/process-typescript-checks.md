# Process TypeScript checks (partial, blocked)

| check | command | result | notes |
|---|---|---|---|
| process-js test target | repo run @semio-tech/process-js:test (MCP run, language en) | FAIL, exit 1 after 1h57m | Graph service never answered ("sent nothing for 120 s"). The project graph slice read spent ~880 s per attempt and waited on a host capacity decision. This is infrastructure, not a test failure. No test counts were produced. |
| typescript-package-commands, dev-route, bridge-hub-fixtures tests | MCP test_file | NOT RUN | The tool call was rejected by the user interruption. |
| settle-continuations law (🧪️tests/🔂️settle-continuations/🟦️.tsx) | MCP test_file | NOT RUN | Rejected. The file exists under the renderer engine tests. |
| plugin-runtime-suite, turn-settle, turn-dispatch typecheck, bridge script.ts tests | n/a | NOT RUN | The file lookup (find) was rejected. |
| hub Rust composition project targets | n/a | NOT RUN | The listing was rejected. |

Fixes made: none. Remaining: everything above needs to be run once the user permits the tool calls again.
