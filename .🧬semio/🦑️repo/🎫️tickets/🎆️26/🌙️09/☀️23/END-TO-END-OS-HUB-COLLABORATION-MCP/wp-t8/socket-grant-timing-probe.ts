import { proveScopedDirectorySocketRevocationFixture } from "/Users/ueli/Documents/semio/🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🧾️fixture-verification/🟦️.ts";
import { assertSocketGrantNativeLawSources, socketGrantNativeLawPlan } from "/Users/ueli/Documents/semio/🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/📋️native-law-plan/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
for (let i = 0; i < 2; i++) {
  let t = performance.now(); await proveScopedDirectorySocketRevocationFixture(root); console.log("fixture", Math.round(performance.now() - t));
  t = performance.now(); assertSocketGrantNativeLawSources(root); console.log("sources", Math.round(performance.now() - t));
  t = performance.now(); socketGrantNativeLawPlan(); console.log("plan", Math.round(performance.now() - t));
}
