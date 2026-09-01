/** mutation payload — mirrors `CreateSubject`. */
import type { Subject } from "../../🟦️component.ts";

export interface CreateSubject {
  subject: Subject;
  index?: number;
}
