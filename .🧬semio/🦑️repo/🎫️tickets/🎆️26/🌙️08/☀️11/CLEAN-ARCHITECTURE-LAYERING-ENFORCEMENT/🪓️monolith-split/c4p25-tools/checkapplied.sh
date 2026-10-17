#!/bin/zsh
# usage: checkapplied.sh <label> [name...]   --check-applied of the three c4p25 artifact trees (log per tree, exit codes in c4p25-checkapplied-<label>.exit)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/c4p25
label=$1; shift
typeset -A P
P=(energymodel "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1" energyengine "✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine" remodeling "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1")
names=("$@"); [ ${#names} -eq 0 ] && names=(energymodel energyengine remodeling)
cd /Users/ueli/Documents/semio || exit 2
ulimit -n 65536
for n in $names; do
  rm -rf "$SCR/ca-$label-$n"; mkdir -p "$SCR/ca-$label-$n"
  perl -e 'alarm 3300; exec @ARGV' -- bun ./📜️script.ts new crates "${P[$n]}" --check-applied --base=f856dff0813 --sandbox="$SCR/ca-$label-$n" > "$F/🗑️generated/c4p25-checkapplied-$label-$n.log" 2>&1
  echo "$n exit=$?" >> "$F/🗑️generated/c4p25-checkapplied-$label.exit"
done
