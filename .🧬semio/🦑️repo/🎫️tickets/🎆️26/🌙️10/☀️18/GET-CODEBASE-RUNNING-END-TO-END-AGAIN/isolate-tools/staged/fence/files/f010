/** 🧪️ Test preload: fences the live store of the checkout holding this script, so no test reads or writes the repository's own images, caches or leases; private repositories below `.cache/test-sandbox` stay open. */
import {repositoryCheckoutRootV1} from "../🧭️root/🟦️.ts";
import {fenceRepositoryStoreV1} from "./🟦️.ts";

const checkout=repositoryCheckoutRootV1(import.meta.dir);
if(checkout!==null)fenceRepositoryStoreV1(checkout);
