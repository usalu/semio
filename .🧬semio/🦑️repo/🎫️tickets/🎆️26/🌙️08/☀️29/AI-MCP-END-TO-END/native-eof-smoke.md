# MCP EOF Smoke Results

`native-eof-smoke.ts` sends raw MCP initialize and initialized frames, closes stdin, and records the actual process status. Each process uses a disposable ticket workspace. The three runs on sandboxed macOS exercise different parts of the launcher.

| Path | Initialize | Result after EOF |
| --- | --- | --- |
| Real TypeScript gateway with deliberately pending stage and the rebuilt native executable set as peer | `semio` `warming` | Status 0, no signal, about 8 ms. The stage observed cancellation and the gateway never selected the peer. |
| Real rebuilt native server with the configured catalog and host policies | `semio` `0.1.0` | Status 0, no signal, about 907 ms. The native server bound the disposable workspace and continued over stdio after the sandbox denied its loopback listener. |
| Configured `repo dev mcp stdio os`, no executable override | `semio` `warming` | Status 1, no signal, about 8 ms. The sandbox denied `sysctl -n kern.bootsessionuuid` during native publication admission before EOF. EOF cancelled the dependency staging child. The launcher still reported the preexisting admission error. |

The gateway run proves cancellation of an outstanding stage at the real gateway boundary. The pending stage was controlled by the test and did not substitute a server. The default launcher run separately exercised the actual failing host admission path and did not start a native peer. It does not establish the status of configured EOF on a host where that admission succeeds.
