"""🧱️ Moves AppRole into app-surface, PackageRole and the media classification enums into view-model."""
import pathlib
M = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest")
pv = (M / "🧳️product-vocabulary/🦀️.rs").read_text()

def between(text, start, end):
    assert text.count(start) == 1 and text.count(end) == 1, (start[:60], end[:60])
    i = text.index(start)
    j = text.index(end, i)
    return text[i:j]

def rep(s, old, new):
    assert s.count(old) == 1, (s.count(old), old[:80])
    return s.replace(old, new)

app_role = between(pv, "/// 👁️✏️ Whether a surface may change the artifact it is bound to.\n", "//#endregion 🔖️Surface\n//#endregion 🔖️Manifest\n// #endregion 🛂️Manifest\n//#endregion 🔖️AppRole\n")
package_role = between(pv, "/// 🎭️ Which actor-world role a package fills", "//#endregion 🔖️PackageDescriptor\n//#endregion 🔖️Manifest\n// #endregion 🛂️Manifest\n//#endregion 🔖️PackageRole\n")
os_media = between(pv, "/// 🧬️ Which geometry backend a resource kind's media exporters/importers target", "//#endregion ArtifactKind\n\n//#region MediaType\n")
media_type = between(pv, "/// 🧬️ Typed-media lattice:", "//#endregion MediaType\n//#endregion 🔖️MediaVocabulary\n")

surface = M / "🧱️app-surface/🦀️.rs"
s = surface.read_text()
s = rep(s, "use super::*;\n", "use super::*;\nuse crate::onboarding::{IntroductionDefinition, TutorialDefinition};\n")
s = rep(s, "//#region 🔖️Surface\n\n/// 🎯️ A surface addressed across plugin boundaries.\n", "//#region 🔖️Surface\n" + app_role + "\n/// 🎯️ A surface addressed across plugin boundaries.\n")
s = rep(s, "/// identically to the introduction walkthrough box, see `ui_react`'s `GLASS_OVERLAY_BOX_CLASS`)\n", "/// identically to every glass overlay box, see `ui_react`'s `GLASS_OVERLAY_BOX_CLASS`)\n")
s = rep(s, "    /// window kind claims as its own, including the framework-injected History/Clipboard/tutorial\n    /// constants.", "    /// window kind claims as its own, including the framework-injected History/Clipboard verbs and\n    /// every reserved contribution.")
surface.write_text(s)

vm = M / "🖼️view-model/🦀️.rs"
v = vm.read_text()
v = rep(v, "//#region 🔖️PackageDescriptor\n", "//#region 🔖️PackageDescriptor\n" + package_role + "\n")
v = rep(v, "//#region ArtifactKind\n", "//#region ArtifactKind\n" + os_media)
v = rep(v, "//#region MediaType\n", "//#region MediaType\n" + media_type)
vm.write_text(v)
print(len(app_role.splitlines()), len(package_role.splitlines()), len(os_media.splitlines()), len(media_type.splitlines()))
