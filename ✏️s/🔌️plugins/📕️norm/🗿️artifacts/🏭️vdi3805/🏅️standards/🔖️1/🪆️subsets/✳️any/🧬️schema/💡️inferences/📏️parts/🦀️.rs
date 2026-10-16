//! 📏️ VDI 3805 sheet parts: the `define_vdi_part!` macro, part 1 and the generated part modules 2 to 100.

use super::*;

//#region 🔖️ComplianceReport
// Preserve part_N modules as thin applicability wrappers for existing tests.
macro_rules! define_vdi_part {
    ($module:ident, $num:literal, reserved) => {
        pub mod $module {
            use super::*;
            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }
            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
                let claimed = document.catalog.products.iter().any(|p| p.sheet.0 == $num);
                if claimed {
                    return CheckResult::assess(format!("vdi3805.{}.reserved", $num), part_label($num), clause(stringify!($num), "scope"), subject_dataset(), copy("Reserved sheet", "Reserviertes Blatt"))
                        .annex(ANNEX)
                        .status(CheckStatus::Fail)
                        .explanation(copy(
                            format!("Products claim reserved sheet {}.", $num),
                            format!("Produkte beanspruchen reserviertes Blatt {}.", $num),
                        ))
                        .remedy(Remedy::exactly(
                            SubjectRef::new("", "catalog.products[0].sheet", copy("Product sheet", "Produktblatt")),
                            Quantity::new(QuantityKind::Dimensionless, $num as f64),
                            Quantity::new(QuantityKind::Dimensionless, 2.0),
                            copy("Move products off reserved sheet numbers.", "Produkte von reservierten Blattnummern verschieben."),
                        ))
                        .build();
                }
                CheckResult::assess(format!("vdi3805.{}.reserved", $num), part_label($num), clause(stringify!($num), "scope"), subject_dataset(), copy("Reserved sheet", "Reserviertes Blatt"))
                    .annex(ANNEX)
                    .not_applicable(copy(format!("sheet {} reserved", $num), format!("Blatt {} reserviert", $num)))
                    .build()
            }
        }
    };
    ($module:ident, $num:literal, historical) => {
        pub mod $module {
            use super::*;
            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }
            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
                let has = document.catalog.products.iter().any(|p| p.sheet.0 == $num);
                if !has {
                    return CheckResult::assess(format!("vdi3805.{}.historical", $num), part_label($num), clause(stringify!($num), "status"), subject_dataset(), copy("Historical proposal sheet", "Historischer Vorschlagsblatt"))
                        .annex(ANNEX)
                        .not_applicable(copy("No product on historical sheet.", "Kein Produkt auf historischem Blatt."))
                        .build();
                }
                evaluate(document).checks.into_iter().find(|c| c.id.contains(&format!(".{}.historical", $num))).unwrap_or_else(|| {
                    CheckResult::assess(format!("vdi3805.{}.historical", $num), part_label($num), clause(stringify!($num), "status"), subject_dataset(), copy("Historical proposal sheet", "Historischer Vorschlagsblatt"))
                        .annex(ANNEX)
                        .status(CheckStatus::Pass)
                        .build()
                })
            }
        }
    };
    ($module:ident, $num:literal, multi_profile) => {
        pub mod $module {
            use super::*;
            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }
            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
                if document.catalog.product_for_sheet(SheetId($num)).is_none() {
                    return CheckResult::assess(format!("vdi3805.{}.profile", $num), part_label($num), clause(stringify!($num), "profile"), subject_dataset(), copy("Edition profile", "Editionsprofil"))
                        .annex(ANNEX)
                        .not_applicable(copy(format!("No product for sheet {}.", $num), format!("Kein Produkt für Blatt {}.", $num)))
                        .build();
                }
                let profile = profile_for_sheet(document, $num);
                let product = document.catalog.product_for_sheet(SheetId($num)).expect("product");
                let keys = sheet_mandatory_keys($num, profile);
                let kv = record_kv(&product.records);
                let missing: Vec<_> = keys
                    .iter()
                    .copied()
                    .filter(|key| match *key {
                        "product_group" => product.identity.product_group.is_empty() && !kv.contains_key("product_group"),
                        other => !kv.get(other).is_some_and(|v| !v.is_empty()),
                    })
                    .collect();
                if missing.is_empty() {
                    CheckResult::assess(format!("vdi3805.{}.profile", $num), part_label($num), clause(stringify!($num), "profile"), subject_dataset(), copy("Edition profile", "Editionsprofil"))
                        .annex(ANNEX)
                        .status(CheckStatus::Pass)
                        .explanation(copy(format!("sheet {} profile {:?} ok", $num, profile), format!("Blatt-{}-Profil {:?} ok", $num, profile)))
                        .build()
                } else {
                    CheckResult::assess(format!("vdi3805.{}.profile", $num), part_label($num), clause(stringify!($num), "profile"), subject_dataset(), copy("Edition profile", "Editionsprofil"))
                        .annex(ANNEX)
                        .status(CheckStatus::Fail)
                        .explanation(copy(format!("profile {:?} missing {:?}", profile, missing), format!("Profil {:?} fehlt {:?}", profile, missing)))
                        .remedy({
                            let first = missing.first().copied().unwrap_or("product_group");
                            let path = if first == "product_group" {
                                product_path(&product.id, "identity.productGroup")
                            } else {
                                let entries = match &product.configuration.attributes {
                                    SheetAttributes::Generic(g) => g.entries.clone(),
                                    _ => Vec::new(),
                                };
                                generic_entry_value_path(&product.id, &entries, first)
                            };
                            Remedy::exactly(
                                SubjectRef::new(&product.id, path, copy("Edition mandatory attribute", "Editions-Pflichtattribut")),
                                Quantity::new(QuantityKind::Dimensionless, 0.0),
                                Quantity::new(QuantityKind::Dimensionless, 1.0),
                                copy(
                                    format!("Set `{first}` to satisfy {:?} keys: {}", profile, keys.join(", ")),
                                    format!("`{first}` setzen für {:?}: {}", profile, keys.join(", ")),
                                ),
                            )
                        })
                        .build()
                }
            }
        }
    };
    ($module:ident, $num:literal) => {
        pub mod $module {
            use super::*;
            pub fn metadata() -> &'static SheetEntry {
                &SHEET_ENTRIES[$num - 1]
            }
            pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
                if let Some(product) = document.catalog.product_for_sheet(SheetId($num)) {
                    let checks = check_sheet_product(document, product);
                    if let Some(fail) = checks.iter().find(|c| c.status == CheckStatus::Fail) {
                        return fail.clone();
                    }
                    if let Some(pass) = checks.into_iter().find(|c| c.status == CheckStatus::Pass) {
                        return pass;
                    }
                    return CheckResult::assess(
                        format!("vdi3805.{}.na", $num),
                        part_label($num),
                        clause(stringify!($num), "scope"),
                        subject_product(product),
                        copy("Sheet without applicable checks", "Blatt ohne anwendbare Prüfung"),
                    )
                    .annex(ANNEX)
                    .not_applicable(copy(format!("No applicable Pass/Fail for sheet {}.", $num), format!("Keine anwendbare Prüfung für Blatt {}.", $num)))
                    .build();
                }
                CheckResult::assess(format!("vdi3805.{}.na", $num), part_label($num), clause(stringify!($num), "scope"), subject_dataset(), copy("Sheet without product", "Blatt ohne Produkt"))
                    .annex(ANNEX)
                    .not_applicable(copy(format!("No product for sheet {}.", $num), format!("Kein Produkt für Blatt {}.", $num)))
                    .build()
            }
        }
    };
}

pub mod part_1 {
    use crate::*;
    pub fn metadata() -> &'static SheetEntry {
        &SHEET_ENTRIES[0]
    }
    pub fn check(document: &Vdi3805Snapshot) -> CheckResult {
        check_part1(document).into_iter().find(|c| c.status == CheckStatus::Fail).unwrap_or_else(|| {
            CheckResult::assess("vdi3805.1.structure", part_label(1), clause("1", "structure"), subject_dataset(), copy("Part 1 structure", "Teil-1-Struktur"))
                .annex(ANNEX)
                .status(CheckStatus::Pass)
                .explanation(copy("Part 1 structure valid", "Teil-1-Struktur gültig"))
                .build()
        })
    }
}

define_vdi_part!(part_02, 2);

define_vdi_part!(part_03, 3);

define_vdi_part!(part_04, 4);

define_vdi_part!(part_05, 5);

define_vdi_part!(part_06, 6);

define_vdi_part!(part_07, 7);

define_vdi_part!(part_08, 8, multi_profile);

define_vdi_part!(part_09, 9);

define_vdi_part!(part_10, 10, multi_profile);

define_vdi_part!(part_11, 11);

define_vdi_part!(part_12, 12, historical);

define_vdi_part!(part_13, 13, historical);

define_vdi_part!(part_14, 14, multi_profile);

define_vdi_part!(part_15, 15, reserved);

define_vdi_part!(part_16, 16);

define_vdi_part!(part_17, 17);

define_vdi_part!(part_18, 18, multi_profile);

define_vdi_part!(part_19, 19);

define_vdi_part!(part_20, 20);

define_vdi_part!(part_21, 21);

define_vdi_part!(part_22, 22);

define_vdi_part!(part_23, 23);

define_vdi_part!(part_24, 24);

define_vdi_part!(part_25, 25, historical);

define_vdi_part!(part_26, 26);

define_vdi_part!(part_27, 27);

define_vdi_part!(part_28, 28);

define_vdi_part!(part_29, 29);

define_vdi_part!(part_30, 30, reserved);

define_vdi_part!(part_31, 31, reserved);

define_vdi_part!(part_32, 32);

define_vdi_part!(part_33, 33, multi_profile);

define_vdi_part!(part_34, 34);

define_vdi_part!(part_35, 35);

define_vdi_part!(part_36, 36, multi_profile);

define_vdi_part!(part_37, 37, multi_profile);

define_vdi_part!(part_38, 38);

define_vdi_part!(part_39, 39, reserved);

define_vdi_part!(part_40, 40, multi_profile);

define_vdi_part!(part_41, 41);

define_vdi_part!(part_42, 42, multi_profile);

define_vdi_part!(part_43, 43);

define_vdi_part!(part_44, 44);

define_vdi_part!(part_45, 45);

define_vdi_part!(part_46, 46, reserved);

define_vdi_part!(part_47, 47, reserved);

define_vdi_part!(part_48, 48, reserved);

define_vdi_part!(part_49, 49, reserved);

define_vdi_part!(part_50, 50);

define_vdi_part!(part_51, 51);

define_vdi_part!(part_52, 52);

define_vdi_part!(part_53, 53, multi_profile);

define_vdi_part!(part_54, 54);

define_vdi_part!(part_55, 55);

define_vdi_part!(part_56, 56, reserved);

define_vdi_part!(part_57, 57, reserved);

define_vdi_part!(part_58, 58, reserved);

define_vdi_part!(part_59, 59, reserved);

define_vdi_part!(part_60, 60);

define_vdi_part!(part_61, 61);

define_vdi_part!(part_62, 62);

define_vdi_part!(part_63, 63);

define_vdi_part!(part_64, 64);

define_vdi_part!(part_65, 65);

define_vdi_part!(part_66, 66);

define_vdi_part!(part_67, 67, reserved);

define_vdi_part!(part_68, 68, reserved);

define_vdi_part!(part_69, 69, reserved);

define_vdi_part!(part_70, 70, reserved);

define_vdi_part!(part_71, 71, reserved);

define_vdi_part!(part_72, 72, reserved);

define_vdi_part!(part_73, 73, reserved);

define_vdi_part!(part_74, 74, reserved);

define_vdi_part!(part_75, 75, reserved);

define_vdi_part!(part_76, 76, reserved);

define_vdi_part!(part_77, 77, reserved);

define_vdi_part!(part_78, 78, reserved);

define_vdi_part!(part_79, 79, reserved);

define_vdi_part!(part_80, 80, reserved);

define_vdi_part!(part_81, 81, reserved);

define_vdi_part!(part_82, 82, reserved);

define_vdi_part!(part_83, 83, reserved);

define_vdi_part!(part_84, 84, reserved);

define_vdi_part!(part_85, 85, reserved);

define_vdi_part!(part_86, 86, reserved);

define_vdi_part!(part_87, 87, reserved);

define_vdi_part!(part_88, 88, reserved);

define_vdi_part!(part_89, 89, reserved);

define_vdi_part!(part_90, 90, reserved);

define_vdi_part!(part_91, 91, reserved);

define_vdi_part!(part_92, 92, reserved);

define_vdi_part!(part_93, 93, reserved);

define_vdi_part!(part_94, 94, reserved);

define_vdi_part!(part_95, 95, reserved);

define_vdi_part!(part_96, 96, reserved);

define_vdi_part!(part_97, 97, reserved);

define_vdi_part!(part_98, 98, reserved);

define_vdi_part!(part_99, 99);

define_vdi_part!(part_100, 100, multi_profile);
//#endregion 🔖️ComplianceReport
