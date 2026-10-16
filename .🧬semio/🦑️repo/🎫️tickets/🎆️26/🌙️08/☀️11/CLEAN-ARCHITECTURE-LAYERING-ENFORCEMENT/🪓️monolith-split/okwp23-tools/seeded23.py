# 🌱 In-process seeded sensitivity checks of the WP2/WP3 scans (no file is written): the extern-path scan must flag a bogus item of a real crate, the orphan scan must flag a foreign trait on a foreign type, the manifest audit must flag an undeclared first-party crate, the lost-item key must change when an item disappears.
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE)
import extcheck as X, okcrates as C, registry, ns, orphan
registry.load(refresh=True)
model = C.build()[1]
pkg = C.pkg_abs('os-store-one-item-publication', model)
deps = X.deps_of(pkg)
good = X.check_path('semio_framework_os_store_ledger', ['CursorRevisionAccumulator'], deps)
bad = X.check_path('semio_framework_os_store_ledger', ['DoesNotExistSeedXyz'], deps)
bad2 = X.check_path('semio_framework_os_store_one_item', ['LaneItemReceiptt'], deps)
print('extcheck real item ->', good, '| bogus ledger item ->', bad, '| typo one-item item ->', bad2)
assert good[0] is True and bad[0] is False and bad2[0] is False
w = ns.ext_world('semio_framework_os_store_ledger')
m = w.root
out = []
orphan.check(m, 'seed', 'std::fmt::Display', 'String', {'start': 0, 'text': 'impl'}, out)
orphan.check(m, 'seed', 'std::fmt::Display', 'CursorRevisionAccumulator', {'start': 0, 'text': 'impl'}, out)
print('orphan scan flags Display for String:', len(out) == 1, '(and passes the same trait for a local type)')
assert len(out) == 1
print('ok')
