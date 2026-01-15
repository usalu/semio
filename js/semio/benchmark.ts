// js/semio/benchmark.ts
import DiffForward from "../../assets/semio/diff_kit_metabolism.json";
import DiffInverse from "../../assets/semio/diff_kit_metabolism_inverted.json";
import InvalidKit from "../../assets/semio/kit_invalid.json";
import MetabolismKit from "../../assets/semio/kit_metabolism.json";
import {
    applyKitDiff,
    deserializeKit,
    flattenDesign,
    Kit,
    KitDiff,
    serializeKit,
    validateKit
} from "./semio.js";

const ITERATIONS = 100;

function bench(name: string, fn: () => void) {
    const start = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        fn();
    }
    const end = performance.now();
    const durationSec = (end - start) / 1000 / ITERATIONS;
    console.log(`${name},${durationSec.toFixed(6)}`);
}

function findDesign(kit: Kit, name: string, parentName?: string) {
    let parentGuid: string | undefined;
    if (parentName) {
        const p = kit.designs?.find(d => d.name === parentName);
        if (!p) throw new Error(`Parent ${parentName} not found`);
        parentGuid = p.guid;
    }
    const d = kit.designs?.find(d => d.name === name && (parentGuid ? d.parent?.guid === parentGuid : !d.parent));
    if (!d) throw new Error(`Design ${name} not found`);
    return d;
}

const kitMetabolism = MetabolismKit as unknown as Kit;
const kitInvalid = InvalidKit as unknown as Kit;
const diffForward = DiffForward as unknown as KitDiff;
const diffInverse = DiffInverse as unknown as KitDiff;

// 1. Roundtrip/Metabolism
bench("Roundtrip/Metabolism", () => {
    const json = serializeKit(kitMetabolism);
    deserializeKit(json);
});

// 2. Diff/Metabolism
bench("Diff/Metabolism", () => {
    const k2 = applyKitDiff(kitMetabolism, diffForward);
    applyKitDiff(k2, diffInverse);
});

// 3. Flatten Design/Nakagin Capsule Tower
const d1 = findDesign(kitMetabolism, "Nakagin Capsule Tower");
bench("Flatten Design/Nakagin Capsule Tower", () => {
    flattenDesign(kitMetabolism, d1.guid);
});

// 4. Flatten Design/Nakagin Capsule Tower/Slanted
// Go test logic: parent is Nakagin.
const d2 = findDesign(kitMetabolism, "Slanted", "Nakagin Capsule Tower");
bench("Flatten Design/Nakagin Capsule Tower/Slanted", () => {
    flattenDesign(kitMetabolism, d2.guid);
});

// 5. Flatten Design/Nakagin Capsule Tower/Twisted
const d3 = findDesign(kitMetabolism, "Twisted", "Nakagin Capsule Tower");
bench("Flatten Design/Nakagin Capsule Tower/Twisted", () => {
    flattenDesign(kitMetabolism, d3.guid);
});

// 6. Flatten Design/Nakagin Capsule Tower/Dancing
const d4 = findDesign(kitMetabolism, "Dancing", "Nakagin Capsule Tower");
bench("Flatten Design/Nakagin Capsule Tower/Dancing", () => {
    flattenDesign(kitMetabolism, d4.guid);
});

// 7. Flatten Design/Capsule Dream
const d5 = findDesign(kitMetabolism, "Capsule Dream");
bench("Flatten Design/Capsule Dream", () => {
    flattenDesign(kitMetabolism, d5.guid);
});

// 8. Validation/Invalid Kit
bench("Validation/Invalid Kit", () => {
    validateKit(kitInvalid);
});

// 9. Validation/Metabolism
bench("Validation/Metabolism", () => {
    validateKit(kitMetabolism);
});

