import type { SemioSnapshot } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioMutation } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🟦️.ts";
export const probe = (snapshot: SemioSnapshot, mutation: SemioMutation): string => `${snapshot.subset.subset}:${mutation.mutation}`;
