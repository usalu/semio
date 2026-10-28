import sys
p=sys.argv[1]+"/files/prefetch-booking.ts"
s=open(p,encoding="utf-8").read()
old=''' expect(method).toContain("await upgradeBuildCapacityBeforeRebuildV1();");'''
new=''' expect(source,"the upgrade before the custody (rev3-g/upgrade-before-custody) lands first").toContain("import{markBuildCapacityPhaseV1,upgradeBuildCapacityBeforeRebuildV1}from");
 expect(method).toContain("await upgradeBuildCapacityBeforeRebuildV1();");'''
assert old in s
s=s.replace(old,new)
open(p,"w",encoding="utf-8").write(s)
