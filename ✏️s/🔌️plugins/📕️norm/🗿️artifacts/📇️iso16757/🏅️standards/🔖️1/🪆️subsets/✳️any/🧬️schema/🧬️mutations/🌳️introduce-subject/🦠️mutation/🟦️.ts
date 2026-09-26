/** mutation payload — mirrors `IntroduceSubject`. */
import type { Subject } from "../../🟦️.ts";

export interface IntroduceSubject {
  subject: Subject;
  index?: number;
}
