# Raster JavaScript Failure Audit

Read-only inspection of bounded Nx terminal output artifacts. Cache files are neither changed nor deleted. A cached failure is supporting diagnostic evidence, not a new test execution.

The terminal artifact for run 7522 records 237 passing and five failing tests across twenty files. All five failures are Ajv strict-schema rejections of the declared x-semio-execution annotation in merge-down and flatten-layers command schemas. Both test validators now explicitly register that annotation vocabulary and retain Ajv strict validation of command bodies; malformed-target and independent Sharp composite assertions remain in place. A fresh supported Raster JavaScript target is running to verify the repair.

## .cache/b/r/m/nx/artifacts/terminalOutputs/2654854906060531968

Modified: 2026-11-05T02:35:28.803Z; bytes: 53732.

```text
159: 173 |     if (mode === true)
160: 174 |         throw new Error(msg);
161:                         ^
162: error: strict mode: unknown keyword: "x-semio-execution"
163:       at checkStrictMode (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/util.js:174:19)
164:       at checkUnknownRules (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/util.js:32:13)
165:       at checkKeywords (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/validate/index.js:120:16)
166:       at validateFunctionCode (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/validate/index.js:19:9)
167:       at compileSchema (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/index.js:80:24)
168:       at _compileSchemaEnv (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/core.js:473:37)
169:       at <anonymous> (/home/deck/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🫳️merge-down/🧪️tests/🟦️.ts:12:20)
170: ✗ Root siblings retain shared image assets [9.51ms]
171: 169 | function checkStrictMode(it, msg, mode = it.opts.strictSchema) {
172: 170 |     if (!mode)
173: 171 |         return;
174: 172 |     msg = `strict mode: ${msg}`;
175: 173 |     if (mode === true)
176: 174 |         throw new Error(msg);
177:                         ^
178: error: strict mode: unknown keyword: "x-semio-execution"
179:       at checkStrictMode (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/util.js:174:19)
180:       at checkUnknownRules (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/util.js:32:13)
181:       at checkKeywords (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/validate/index.js:120:16)
182:       at validateFunctionCode (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/validate/index.js:19:9)
183:       at compileSchema (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/index.js:80:24)
184:       at _compileSchemaEnv (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/core.js:473:37)
185:       at <anonymous> (/home/deck/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🫳️merge-down/🧪️tests/🟦️.ts:12:20)
186: ✗ Nested siblings preserve the parent transform and opacity [12.26ms]
187: 169 | function checkStrictMode(it, msg, mode = it.opts.strictSchema) {
188: 170 |     if (!mode)
189: 171 |         return;
190: 172 |     msg = `strict mode: ${msg}`;
191: 173 |     if (mode === true)
192: 174 |         throw new Error(msg);
193:                         ^
194: error: strict mode: unknown keyword: "x-semio-execution"
195:       at checkStrictMode (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/util.js:174:19)
196:       at checkUnknownRules (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/util.js:32:13)
197:       at checkKeywords (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/validate/index.js:120:16)
198:       at validateFunctionCode (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/validate/index.js:19:9)
199:       at compileSchema (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/index.js:80:24)
200:       at _compileSchemaEnv (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/core.js:473:37)
201:       at <anonymous> (/home/deck/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🫳️merge-down/🧪️tests/🟦️.ts:12:20)
202: ✗ Masked translucent group merges without losing coverage [7.83ms]
203: ✓ Missing selection refuses merge [0.35ms]
204: ✓ Bottom layer refuses merge [0.03ms]
205: ✓ Hidden upper refuses merge [0.06ms]
206: ✓ Backdrop blend refuses merge [0.02ms]
207: ✓ Adjustment sibling refuses merge [0.02ms]
217: 119 |         // console.log("\n\n\n *** \n", sourceCode, this.opts)
218: 120 |         throw e;
219:                     ^
220: error: strict mode: unknown keyword: "x-semio-execution"
221:       at compileSchema (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/index.js:120:15)
222:       at _compileSchemaEnv (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/core.js:473:37)
223:       at <anonymous> (/home/deck/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🫳️merge-down/🧪️tests/🟦️.ts:29:28)
224: ✗ merge command rejects malformed targets [10.91ms]
225: ✓ Inherited parent lock prevents merging siblings [0.22ms]
226: ✓ A protected descendant prevents baking its group [0.02ms]
227: 
228: ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥞️flatten-layers/🧪️tests/🟦️.ts:
229: 169 | function checkStrictMode(it, msg, mode = it.opts.strictSchema) {
233: 173 |     if (mode === true)
234: 174 |         throw new Error(msg);
235:                         ^
236: error: strict mode: unknown keyword: "x-semio-execution"
237:       at checkStrictMode (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/util.js:174:19)
238:       at checkUnknownRules (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/util.js:32:13)
239:       at checkKeywords (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/validate/index.js:120:16)
240:       at validateFunctionCode (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/validate/index.js:19:9)
241:       at compileSchema (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/compile/index.js:80:24)
242:       at _compileSchemaEnv (/home/deck/semio/.cache/b/d/m/j/70302f586ef25ec8c8c8/node_modules/ajv/dist/core.js:473:37)
243:       at <anonymous> (/home/deck/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥞️flatten-layers/🧪️tests/🟦️.ts:9:28)
244: ✗ flatten command preserves visible appearance and placement [9.92ms]
245: 
246: ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧱️preparation/🟦️.ts:
247: ✓ shared source image preparation preserves the composite [43.37ms]
248: 
249: ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🎛️adjustment/🟦️.ts:
418: ✓ persistent selection coverage adjacent-single-pixels matches Sharp [0.60ms]
419: ✓ persistent selection coverage paged-prefix matches Sharp [0.65ms]
420: 
421: 5 tests failed:
422: ✗ Root siblings retain shared image assets [9.51ms]
423: ✗ Nested siblings preserve the parent transform and opacity [12.26ms]
424: ✗ Masked translucent group merges without losing coverage [7.83ms]
425: ✗ merge command rejects malformed targets [10.91ms]
426: ✗ flatten command preserves visible appearance and placement [9.92ms]
427: 
428:  237 pass
429:  5 fail
430:  850 expect() calls
431: Ran 242 tests across 20 files. [1.63s]
436: 185 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
437: 186 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
438:                                                                  ^
439: OwnedCommandFailure: raster-owning-tests failed (1)
440:  outcome: {
441:   version: 1,
442:   status: 1,
443:   signal: null,
444: },
457: 185 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
458: 186 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
459:                                                                  ^
460: OwnedCommandFailure: process:owner-command failed (1)
461:  outcome: {
462:   version: 1,
463:   status: 1,
464:   signal: null,
465: },
```
