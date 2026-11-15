import { animal } from "./author-05-animal.ts";

animal("labrador-dog", "dog", "Hund", [{ match: /^(mane|forelock|tongue|inner-ear)/, fit: { p: 0.3 } }]);
