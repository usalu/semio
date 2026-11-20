import sys
o,src=sys.argv[1],sys.argv[2]
s=open(o,encoding='utf-8').read()
for a,b in [("fn original_ui_arena_opening_and_registration_keep_actual_lifetime_owners(){let c:","fn original_ui_arena_opening_and_registration_keep_actual_lifetime_owners(){let _serial=ui_arena_serial();let c:"),
            ("fn arena_opening_quote_matches_actual_target_types_allocations_and_finite_work(){static QUOTE_REGISTRY:UiArenaRegistry=UiArenaRegistry::new();","fn arena_opening_quote_matches_actual_target_types_allocations_and_finite_work(){static QUOTE_REGISTRY:UiArenaRegistry=UiArenaRegistry::new();let _serial=ui_arena_serial();")]:
    assert s.count(a)==1,a;s=s.replace(a,b)
open(o,'w',encoding='utf-8').write(s)
s=open(src,encoding='utf-8').read()
a="fn ui_arena_lifetime_owns_every_partial_allocation_before_publication(){\n let corpus"
assert s.count(a)==1;s=s.replace(a,"fn ui_arena_lifetime_owns_every_partial_allocation_before_publication(){\n let _serial=ui_arena_serial();let corpus")
open(src,'w',encoding='utf-8').write(s)
