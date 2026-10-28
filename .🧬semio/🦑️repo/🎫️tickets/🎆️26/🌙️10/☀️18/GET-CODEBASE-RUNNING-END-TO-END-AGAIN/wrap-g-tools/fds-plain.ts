import "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/wrap-g-tools/fds-first.ts";
import "node:child_process";
import {spawnSync} from "node:child_process";
const a=process.argv.slice(2);process.exit(spawnSync(a[0]!,a.slice(1),{stdio:"inherit"}).status??1);
