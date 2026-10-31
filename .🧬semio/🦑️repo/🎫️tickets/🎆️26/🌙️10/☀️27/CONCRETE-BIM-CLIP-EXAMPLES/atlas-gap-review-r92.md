# Diagnostic Ground Coverage Review

The source/producer-moving diagnostic shows24 uncovered positive XY components. These are reviewed against source programmes and actual occupied heights before changing geometry. No material is added solely from a projection gap.

{'area': 3.2971966625354304e-05, 'bounds': [18.8599250000908, 1.1099250000908005, 19.0, 1.3900749999091995], 'point': (18.860112375418698, 1.25457821904611)}
{'area': 3.2971966625375e-05, 'bounds': [18.8599250000908, 5.859925000090801, 19.0, 6.140074999909199], 'point': (18.860112375418694, 6.00457821904611)}
{'area': 3.297196662532854e-05, 'bounds': [18.8599250000908, 10.6099250000908, 19.0, 10.8900749999092], 'point': (18.860112375418694, 10.75457821904611)}
{'area': 3.297196662532854e-05, 'bounds': [18.8599250000908, 15.3599250000908, 19.0, 15.6400749999092], 'point': (18.860112375418694, 15.50457821904611)}
{'area': 3.2971966625341286e-05, 'bounds': [18.8599250000908, 20.1099250000908, 19.0, 20.3900749999092], 'point': (18.860112375418694, 20.25457821904611)}
{'area': 3.297196662537974e-05, 'bounds': [0.0, 20.1099250000908, 0.14007499990919947, 20.3900749999092], 'point': (0.139887624581302, 20.25457821904611)}
{'area': 3.2971966625380216e-05, 'bounds': [0.0, 15.3599250000908, 0.14007499990919947, 15.6400749999092], 'point': (0.139887624581302, 15.50457821904611)}
{'area': 3.2971966625380216e-05, 'bounds': [0.0, 10.6099250000908, 0.14007499990919947, 10.8900749999092], 'point': (0.139887624581302, 10.75457821904611)}
{'area': 3.297196662541095e-05, 'bounds': [0.0, 5.859925000090801, 0.14007499990919947, 6.140074999909199], 'point': (0.139887624581302, 6.00457821904611)}
{'area': 3.297196662541053e-05, 'bounds': [0.0, 1.1099250000908005, 0.14007499990919947, 1.3900749999091995], 'point': (0.139887624581302, 1.25457821904611)}
{'area': 1.5772868925545747, 'bounds': [5.717144828883, 8.250220372263, 7.405663431027955, 11.273247148118642], 'point': (6.536402235736902, 9.769890240778736)}
{'area': 6.59439332507672e-05, 'bounds': [9.3599250000908, 1.1099250000908005, 9.6400749999092, 1.3900749999091995], 'point': (9.360112375418698, 1.25457821904611)}
{'area': 9.159777154769955e-05, 'bounds': [4.584911607607206, 1.0849116076072063, 4.915088392392794, 1.4150883923927937], 'point': (4.58499167405599, 1.2510957228564998)}
{'area': 0.0005337539639769385, 'bounds': [5.717144828883, 8.250220372263, 5.73048867798254, 8.29022037226265], 'point': (5.72381675343277, 8.270220372262825)}
{'area': 6.594393325074638e-05, 'bounds': [14.1099250000908, 5.859925000090801, 14.3900749999092, 6.140074999909199], 'point': (14.110112375418698, 6.00457821904611)}
{'area': 9.1597771544355e-05, 'bounds': [14.084911607607205, 1.0849116076072063, 14.415088392392795, 1.4150883923927937], 'point': (14.08499167405599, 1.2510957228564998)}
{'area': 0.09874999999999437, 'bounds': [16.0, 10.85, 16.025, 14.799999999999999], 'point': (16.0125, 13.8125)}
{'area': 1.899615896087961e-05, 'bounds': [14.263351969123605, 6.615936522366877, 14.4030749999092, 6.7895691173986865], 'point': (14.393992186177313, 6.706699343197867)}
{'area': 0.05925000000000223, 'bounds': [16.06, 10.85, 16.075, 14.799999999999999], 'point': (16.0675, 13.8125)}
{'area': 0.0028254151827425047, 'bounds': [4.7, 1.2, 4.8, 1.3], 'point': (4.705000000000033, 1.250000000001)}
{'area': 0.004297770427971179, 'bounds': [4.713, 1.213, 4.787, 1.287], 'point': (4.75, 1.2512099578907576)}
{'area': 0.0020395406559774827, 'bounds': [6.2745, 9.1545, 6.3255, 9.205499999999999], 'point': (6.3, 9.181249718539203)}
{'area': 0.002825415182734146, 'bounds': [14.2, 1.2, 14.3, 1.3], 'point': (14.205042821535224, 1.2513080625846027)}
{'area': 0.00429777042797116, 'bounds': [14.213, 1.213, 14.287, 1.287], 'point': (14.25, 1.2512099578907576)}

Reviewed classification: the1.5773m² connected component is the central stair opening, with the nearby0.0005338m² stair-edge return. Two long gaps atX16.000–16.025 and16.060–16.075 are the layered garage-wall section cavities, not missing floor material. The0.00203954m² central component is the hollow55mm tube air core. Hollow drain/piloti core components and explicitly inferred conservative exterior column reservations explain the remaining small components. This union of occupied projection bands provides no evidence of an unassigned large hole; it does not prove finish coverage at a chosen exact elevation or depth occlusion.


Correction after material-layer inspection:LS-PARTITION-15 has no air cavity. The two strips match the GARAGE-SLIDING opening spanY10.85–14.80 and the25mm/15mm clearances around its35mm leaf. They require an actual threshold/ground-surface investigation; they must not be passed as wall cavities. No geometry has yet been changed for these strips.


Final garage classification is now confirmed by actual source/native tests and repaired in the existingSL-GROUND product; seegarage-threshold-repair-r92.md. The initial wall-cavity interpretation above is explicitly superseded. The footprint expands only0.075×3.95m within the existing door opening, retaining all previous programme area. The diagnostic image itself remains a moving-source checkpoint and will be regenerated at final freeze.
