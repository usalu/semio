import { ENTWERFEN_MIT_BESTAND_BRAND } from "../../../../../../framework/product/os/dev/brand/index.ts";
import { shouldPersistIntroductionSeen, shouldReplayIntroductionOnLoad } from "../../../../../../framework/renderer/react/index.tsx";
import { readStoredIntroductionSeen, writeStoredIntroductionSeen, UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX } from "../../../../../../ui/js/react/index.tsx";

const storage = new Map<string, string>();
(globalThis as { localStorage?: Storage }).localStorage = {
  getItem: (key) => storage.get(key) ?? null,
  setItem: (key, value) => {
    storage.set(key, String(value));
  },
  removeItem: (key) => {
    storage.delete(key);
  },
  clear: () => storage.clear(),
  key: (index) => [...storage.keys()][index] ?? null,
  get length() {
    return storage.size;
  },
} as Storage;

const key = `${ENTWERFEN_MIT_BESTAND_BRAND.id}:puzzle3d-play`;
writeStoredIntroductionSeen(key);
const seen = readStoredIntroductionSeen(key);
const replay = shouldReplayIntroductionOnLoad(ENTWERFEN_MIT_BESTAND_BRAND);
const persist = shouldPersistIntroductionSeen(ENTWERFEN_MIT_BESTAND_BRAND);
const wouldAutoStart = replay || !seen;

console.log(`[DEBUG] brand=${ENTWERFEN_MIT_BESTAND_BRAND.id}`);
console.log(`[DEBUG] replayIntroductionOnLoad=${ENTWERFEN_MIT_BESTAND_BRAND.replayIntroductionOnLoad}`);
console.log(`[DEBUG] storageKey=${UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}${key} seen=${seen}`);
console.log(`[DEBUG] shouldReplay=${replay} shouldPersist=${persist} wouldAutoStartDespiteSeen=${wouldAutoStart}`);

if (!replay || persist || !wouldAutoStart) {
  console.error("[DEBUG] policy verification failed");
  process.exit(1);
}

console.log("[DEBUG] policy verification passed — Aggregator tour auto-starts after refresh even when previously seen");
