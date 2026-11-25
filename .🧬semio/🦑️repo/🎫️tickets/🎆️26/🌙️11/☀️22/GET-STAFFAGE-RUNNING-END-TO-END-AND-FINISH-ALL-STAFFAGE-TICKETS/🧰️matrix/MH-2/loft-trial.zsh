# 🧵️ Writes one trial job of lofts into $GEN/jobs (source after ../MR/env.sh with BATCH=MH-2): `lofttrial <job> <id> <plan> <name> [scales]` copies the catalog file, applies limbs.ts twice, gates the copy and measures the guard; result in $GEN/jobs/<job>.out.
lofttrial() {
  local j="$1" a="$2" plan="$3" n="$4" sc="${5:-50,100,200,500}" t="$GEN/try/$4.json" P="$S/📦️packages/🟦️typescript"
  { echo "# 800 3600"; echo "cp \"\$(ls \"$S\"/🗂️catalog/*/$a/🔣️.json)\" \"$t\"; cd \"$T\""
    echo "for i in 1 2; do \"$B\" \"$W/limbs.ts\" \"$t\" --plan \"$plan\" --out \"$t\" > \"$GEN/try/$n.limbs.log\" 2>&1; grep REFUSED \"$GEN/try/$n.limbs.log\"; tail -1 \"$GEN/try/$n.limbs.log\" | cut -c1-300; done"
    echo "cd \"$P\"; \"$B\" \"$U/🧰️matrix/MX-2/guard.ts\" \"$t\" --scales $sc 2>&1 | cut -c1-260"
    echo "cd \"$T\"; \"$B\" \"$M/gate.ts\" \"$t\" --scales $sc --twins > \"$GEN/try/$n.gate.log\" 2>&1; grep -E 'HARD|^    [a-z]|^hard' \"$GEN/try/$n.gate.log\" | cut -c1-200"
  } > "$GEN/jobs/$j.job"
}
