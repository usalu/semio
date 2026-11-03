N="nice -n 19 ionice -c3 /home/deck/semio/.venv/bin/python -I"
R=/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts
for c in 1995 1996 1997; do
  $N append_rows.py en$c rows$c.tsv
  f=$(ls -d $R/*en$c)/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🚦️check-outcome-en$c-1
  sed -i 's/ | 1e-2 |$/ | 1e-6 |/' $f/🥒️.feature
  echo en$c; $N ../../🗑️generated/doc-oracles/dry-run.py $f | awk '{print $1}' | sort | uniq -c
done
echo ALLDONE
