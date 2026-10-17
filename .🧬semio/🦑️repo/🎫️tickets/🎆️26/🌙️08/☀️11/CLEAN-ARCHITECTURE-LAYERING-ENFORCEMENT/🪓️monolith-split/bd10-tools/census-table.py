import csv,sys
F='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/bd10-cgc7-crate-census.csv'
for row in csv.DictReader(open(F)):
    if row['name'].startswith('semio-s-artifact-raster-raster') and (row['libFiles']!='0' or row['testFiles']!='0'):
        print(f"{row['name'][27:]:<24} lib {row['libFiles']:>3}/{row['libBytes']:>7}/{row['libLines']:>5} ({row['libRatio']})  test {row['testFiles']:>3}/{row['testBytes']:>7}/{row['testLines']:>5} ({row['testRatio']}) {'LIBOVER ' if row['libOver']=='true' else ''}{'TESTOVER' if row['testOver']=='true' else ''} {row['error'][:60]}")
