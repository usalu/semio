import { composeSpecificOsCatalogV1 } from "../../../✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts";
import { bootFrameworkOsDev } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-2d";
import { DEMONSTRATOR_SHELL_BRANDS } from "../🪧️brand.ts";
await bootFrameworkOsDev({ catalogRows: composeSpecificOsCatalogV1(window.location.href, { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} }).rows, brands: DEMONSTRATOR_SHELL_BRANDS, surfaceSessionFactories: PUZZLE_BOARD_SESSION_FACTORIES });
