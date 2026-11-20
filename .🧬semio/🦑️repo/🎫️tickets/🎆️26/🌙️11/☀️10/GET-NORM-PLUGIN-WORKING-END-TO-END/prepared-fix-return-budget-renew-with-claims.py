"""Prepared, NOT applied. Usage: python3 prepared-fix-return-budget-renew-with-claims.py --check | --apply
Renews the return budget's ledger at every origin refresh even while claims are live, as long as the live holds still fit the new origin's stock."""
import sys
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs"
OLD="if self.origin_prior.is_none()||self.renewable(){self.spent=original.spent();self.work=original.work()}"
NEW="if self.origin_prior.is_none()||self.renewable()||self.holds_fit_origin(&original){self.spent=original.spent();self.work=original.work()}"
ANCHOR=" pub fn renew_turn(&mut self)->Result<(),ValueError>{"
HELPER=''' /// 🏠️ Whether the currency held by live return claims still fits the stock of a freshly refreshed origin, so the ledger can be re-based on that origin without waiting for the claims to settle (held currency stays owned by the outer total).
 fn holds_fit_origin(&self,origin:&I)->bool{let granted=origin.granted();let spent=origin.spent();spent.fits(granted)&&spent.copied_items.checked_add(self.held_items).is_some_and(|items|items<=granted.maximum_items)&&spent.copied_bytes.checked_add(self.held_copy_bytes).is_some_and(|bytes|bytes<=granted.maximum_copy_bytes)&&spent.released_bytes.checked_add(self.held_release_bytes).is_some_and(|bytes|bytes<=granted.maximum_release_bytes)&&spent.retained_capacity_bytes.checked_add(self.held_capacity_bytes).is_some_and(|bytes|bytes<=granted.maximum_capacity_bytes)&&origin.work().checked_add(u128::from(self.held_fuel)).is_some_and(|work|work<=u128::from(origin.fuel()))}
'''
s=open(P,encoding="utf-8").read()
ok=s.count(OLD)==1 and s.count(ANCHOR)==1
print("targets present:",ok)
if "--apply" in sys.argv:
    assert ok
    s=s.replace(OLD,NEW).replace(ANCHOR,HELPER+ANCHOR)
    open(P,"w",encoding="utf-8").write(s);print("applied")
