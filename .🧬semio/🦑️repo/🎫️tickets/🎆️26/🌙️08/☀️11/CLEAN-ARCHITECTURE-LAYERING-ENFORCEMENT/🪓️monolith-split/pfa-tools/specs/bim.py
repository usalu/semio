F = '✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️patches/🦀️.rs'
SPEC = dict(
    orig='bim.rs.txt',
    file=F,
    children=[
        dict(folder='📝️documentation', mod='documentation', ranges=[(176, 489), (1085, 1290), (3425, 3609)],
             doc='//! 📝️ Sparse patches of the documentation records: dimensions, tags, text notes, leaders, annotation styles, sheets, viewports, sheet revisions, views and schedules.'),
        dict(folder='📚️library', mod='library', ranges=[(5, 174), (491, 825), (1418, 1899), (2222, 2313)],
             doc='//! 📚️ Sparse patches of the library records: property templates, classification systems, space conditions, families, components, MEP elements, materials and the element types.'),
        dict(folder='🤝️coordination', mod='coordination', ranges=[(827, 1083)],
             doc='//! 🤝️ Sparse patches of the coordination records: clash sets, rules, issues and issue comments.'),
        dict(folder='🏢️site-and-spaces', mod='site_and_spaces', ranges=[(1292, 1416), (1901, 2127), (3242, 3423)],
             doc='//! 🏢️ Sparse patches of the project, wall sweeps, site, buildings, storeys, grid lines, spaces, zones and area schemes.'),
        dict(folder='🧱️building-elements', mod='building_elements', ranges=[(2129, 2220), (2315, 3240)],
             doc='//! 🧱️ Sparse patches of the physical building elements: walls, curtain walls, columns, beams, slabs, ceilings, roofs, openings, stairs, railings and ramps.'),
        dict(folder='🖋️hand-written', mod='hand_written', ranges=[(3612, 3751)],
             doc='//! 🖋️ Hand-written sparse patches that the model generator does not emit: option groups, design options, worksets, element memberships, structural supports, load cases and structural loads.'),
    ],
    external=[],
)
