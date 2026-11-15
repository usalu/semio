import { animal } from "./author-05-animal.ts";

animal("domestic-cat", "Cat", "Katze", [{ match: /^(mane|forelock|tongue|inner-ear)/, fit: { p: 0.3 } }]);
