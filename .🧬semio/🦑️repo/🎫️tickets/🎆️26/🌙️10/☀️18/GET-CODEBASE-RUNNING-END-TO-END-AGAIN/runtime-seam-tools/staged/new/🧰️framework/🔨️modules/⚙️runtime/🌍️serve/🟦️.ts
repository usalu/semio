import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";
import type { RuntimeFileV1, RuntimeServeOptionsV1, RuntimeServerV1 } from "../📋️contract/🟦️.ts";

/** 📄️ Describes a file for reading or serving. */
export function runtimeFileV1(path: string): RuntimeFileV1 { return requireRuntimeMemberV1("file")(path); }
/** 🌍️ Serves HTTP. */
export function runtimeServeV1(options: RuntimeServeOptionsV1): Promise<RuntimeServerV1> { return requireRuntimeMemberV1("serve")(options); }
