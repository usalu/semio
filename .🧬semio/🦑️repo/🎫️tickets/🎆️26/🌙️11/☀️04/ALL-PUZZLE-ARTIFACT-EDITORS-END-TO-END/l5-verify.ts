import { runVerifyScript } from "/home/deck/semio/✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔬️typescript-package-commands/🟦️.ts";
const ctx={ root:"/home/deck/semio/✏️s/🧑‍💻dev/🧩️puzzle/📦️packages/🟦️typescript", repoRoot:"/home/deck/semio", invocation: undefined as never };
const mode=process.argv[2]!;
await runVerifyScript(ctx,[mode]);
console.log("VERIFY OK",mode);
