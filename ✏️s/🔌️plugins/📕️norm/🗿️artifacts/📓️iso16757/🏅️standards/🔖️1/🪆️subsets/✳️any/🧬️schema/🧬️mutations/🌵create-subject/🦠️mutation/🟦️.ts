/** mutation payload — mirrors `CreateSubject`. */
import type { Subject } from "../../🟦️.ts";

export interface CreateSubject {
  subject: Subject;
  index?: number;
}
