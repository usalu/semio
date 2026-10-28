#!/usr/bin/env python3
"""🔗️ e7: the vouch record law knows the link list."""
import sys
F = sys.argv[1] + "/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🧪️tests/🟦️.ts"
text = open(F, encoding="utf8").read()
old = ''' expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:EPOCH+":4"})).toBe(true);
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:"nope"})).toBe(false);
 expect(check({version:1,consumer:"x",key:"k",cursor:EPOCH+":4",extra:1})).toBe(false);'''
new = ''' expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:EPOCH+":4",linked:[]})).toBe(true);
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:EPOCH+":4",linked:["alias","a/b"]})).toBe(true);
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:EPOCH+":4"}),"a record without its link list").toBe(false);
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:EPOCH+":4",linked:[1]})).toBe(false);
 expect(check({version:1,consumer:"dependencies:.",key:"k",cursor:"nope",linked:[]})).toBe(false);
 expect(check({version:1,consumer:"x",key:"k",cursor:EPOCH+":4",linked:[],extra:1})).toBe(false);'''
assert text.count(old) == 1
open(F, "w", encoding="utf8").write(text.replace(old, new))
