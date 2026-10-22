import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";
import type { RuntimeGlobScanOptionsV1, RuntimeGlobV1 } from "../📋️contract/🟦️.ts";

/** 🌐️ A compiled glob whose implementation is chosen by the hosting runtime. */
export class GlobV1 implements RuntimeGlobV1 {
  private readonly inner: RuntimeGlobV1;
  constructor(pattern: string) { this.inner = requireRuntimeMemberV1("glob")(pattern); }
  match(path: string): boolean { return this.inner.match(path); }
  scan(options?: RuntimeGlobScanOptionsV1): AsyncIterableIterator<string> { return this.inner.scan(options); }
  scanSync(options?: RuntimeGlobScanOptionsV1): IterableIterator<string> { return this.inner.scanSync(options); }
}
