modules 97 (53 eager static, 44 lazy dynamic), source 937 KiB, sum of own import times 106 ms (warm transpile cache, host load applies)

| group | modules | KiB | own import ms |
|---|---:|---:|---:|
| graph | 5 | 85 | 6.9 |
| caller | 5 | 57 | 17.5 |
| invocation | 1 | 25 | 1.4 |
| selection | 2 | 5 | 1.0 |
| resident | 6 | 16 | 3.4 |
| bootstrap | 3 | 12 | 1.2 |
| store-receipt | 1 | 12 | 14.3 |
| revision | 1 | 10 | 2.3 |
| routing | 5 | 69 | 3.8 |
| leases | 6 | 123 | 11.0 |
| host-capacity | 6 | 57 | 5.0 |

| module | self ms | total ms | KiB | kind |
|---|---:|---:|---:|---|
| 🏗️build/🧾️store-receipt/🟨️.mjs | 14.3 | 14.3 | 12.4 | eager |
| 🏗️build/📥️caller/🟦️.ts | 12.3 | 34.6 | 14.9 | eager |
| 🎛️owned-execution/📥️capture/🟦️.ts | 7.8 | 9.6 | 20.5 | eager |
| ♻️retirement/🧭️continuation/🟦️.ts | 4.2 | 4.2 | 5.5 | lazy |
| 🕸️graph/🕰️generations/🟨️.mjs | 3.8 | 3.8 | 44.9 | lazy |
| 🧾️observation/📁️descriptor/🟦️.ts | 3.2 | 4.4 | 166.6 | lazy |
| 🏗️build/📤️arguments/🟨️.mjs | 3.1 | 3.1 | 7.9 | eager |
| 📥️caller/📤️orchestration/🟨️.mjs | 2.8 | 5.9 | 7.5 | eager |
| 📥️inputs/🔏️revision/🟨️.mjs | 2.3 | 2.3 | 10.5 | lazy |
| 💾️storage/🧾️ledger/🟦️.ts | 2.2 | 2.2 | 26.8 | lazy |
| 🏗️build/🌡️host-capacity/🟦️.ts | 2.2 | 5.0 | 7.0 | lazy |
| 🏃️process/🔒️leases/🟦️.ts | 2.1 | 11.7 | 72.5 | eager |
| 🧭️order/🗂️registry/🟦️.ts | 2.1 | 2.1 | 10.2 | eager |
| 📥️invocation/🏃️process/🟦️.ts | 1.9 | 3.2 | 52.8 | eager |

| module (by subtree) | total ms | self ms |
|---|---:|---:|
| 🏗️build/📥️caller/🟦️.ts | 34.6 | 12.3 |
| 📚️library/🏗️build/📜️script.ts | 32.0 | 0.0 |
| 📥️caller/📤️child/🟦️.ts | 29.8 | 0.8 |
| 🟦️typescript/🎛️execution/🟦️.ts | 14.4 | 0.3 |
| 🏗️build/🧾️store-receipt/🟨️.mjs | 14.3 | 14.3 |
| 🏗️build/⚙️operations/🟦️.ts | 13.6 | 0.6 |
| 🟦️typescript/🔒️leases/🟦️.ts | 12.2 | 0.5 |
