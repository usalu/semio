modules 157 (96 eager static, 61 lazy dynamic), source 1476 KiB, sum of own import times 159 ms (warm transpile cache, host load applies)

| group | modules | KiB | own import ms |
|---|---:|---:|---:|
| graph | 5 | 85 | 9.4 |
| native-build | 34 | 332 | 29.2 |
| test-harness | 18 | 111 | 11.2 |
| cargo | 1 | 15 | 2.0 |
| caller | 5 | 57 | 9.9 |
| invocation | 1 | 24 | 1.4 |
| selection | 2 | 5 | 1.0 |
| resident | 6 | 16 | 3.3 |
| bootstrap | 3 | 12 | 1.4 |
| store-receipt | 1 | 12 | 2.1 |
| revision | 1 | 10 | 3.8 |
| routing | 5 | 69 | 4.4 |
| leases | 6 | 123 | 16.6 |
| host-capacity | 6 | 57 | 4.8 |

| module | self ms | total ms | KiB | kind |
|---|---:|---:|---:|---|
| ♻️retirement/🧭️continuation/🟦️.ts | 10.0 | 10.0 | 5.5 | lazy |
| ⚙️runtime/🧭️selection/🟦️.ts | 6.8 | 6.8 | 6.5 | eager |
| 📥️caller/📤️orchestration/🟨️.mjs | 6.8 | 13.1 | 7.5 | eager |
| 🏗️build/📤️arguments/🟨️.mjs | 6.3 | 6.3 | 7.9 | eager |
| 🕸️graph/🕰️generations/🟨️.mjs | 5.2 | 5.2 | 44.9 | lazy |
| 🧾️observation/📁️descriptor/🟦️.ts | 4.5 | 14.4 | 166.6 | lazy |
| 📖️syntax/🦀️rust/🟦️.ts | 4.3 | 4.3 | 45.3 | eager |
| 🧾️observation/📁️current/🟦️.ts | 4.1 | 4.1 | 14.2 | lazy |
| 📥️inputs/🔏️revision/🟨️.mjs | 3.8 | 3.8 | 10.5 | lazy |
| 🧬️schema/✅️validator/🟦️.ts | 3.0 | 3.0 | 15.7 | eager |
| 💾️storage/🧾️ledger/🟦️.ts | 2.5 | 2.5 | 26.8 | lazy |
| 📥️invocation/🏃️process/🟦️.ts | 2.4 | 3.8 | 52.8 | eager |
| ⚙️runtime/🏷️identity/🟦️.ts | 2.3 | 9.1 | 5.7 | eager |
| 🏗️native-build/💾️storage/🟦️.ts | 2.2 | 3.5 | 68.3 | eager |

| module (by subtree) | total ms | self ms |
|---|---:|---:|
| 🏗️build/📥️caller/🟦️.ts | 50.7 | 0.7 |
| 🦀️cargo/🏃️execution/🟦️.ts | 34.1 | 2.0 |
| 📚️library/🏗️build/📜️script.ts | 28.8 | 0.0 |
| 🧪️testing/🦀️cargo/🟦️.ts | 27.4 | 0.2 |
| 🧪️testing/🦀️cargo/🟦️.ts | 26.5 | 1.2 |
| 📥️caller/📤️child/🟦️.ts | 26.1 | 0.8 |
| 📁️filesystem/🧾️observation/🟦️.ts | 18.6 | 0.0 |
