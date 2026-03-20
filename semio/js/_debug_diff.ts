// #region 🔖Header
// [👤semio📚js💻debugdiff](repo://p/u/semio/b/l/js/f/_debug_diff.ts)
// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header

import { MetabolismKit, MetabolismKitDiff, MetabolismKitDiffed } from "@semio/assets";
import { getKitDiff, areKitDiffsEqual, Kit, KitDiff, deserializeKit, parseKitDiff } from "./semio";

const kitOriginal: Kit = deserializeKit(MetabolismKit);
const kitDiffed: Kit = deserializeKit(MetabolismKitDiffed);
const kitDiff: KitDiff = parseKitDiff(MetabolismKitDiff);

const computedDiff = getKitDiff(kitOriginal, kitDiffed);

const allKeys = new Set([...Object.keys(computedDiff), ...Object.keys(kitDiff)]);
for (const key of allKeys) {
  const cVal = JSON.stringify((computedDiff as any)[key]);
  const eVal = JSON.stringify((kitDiff as any)[key]);
  if (cVal !== eVal) {
    console.log(`Key "${key}" DIFFERS:`);
    console.log(`  computed (${cVal?.length}): ${cVal?.substring(0, 1000)}`);
    console.log(`  expected (${eVal?.length}): ${eVal?.substring(0, 1000)}`);
  } else {
    console.log(`Key "${key}" OK (len=${cVal?.length})`);
  }
}
