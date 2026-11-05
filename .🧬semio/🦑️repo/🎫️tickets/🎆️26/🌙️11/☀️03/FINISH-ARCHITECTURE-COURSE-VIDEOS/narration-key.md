# Energy Balance Narration Key

`Beat2_Primaerenergiefaktor` and `Beat4_Referenzgebaeude` called `say(tr("renewable", "erneuerbar"))`. `say` looks up a narration id. English `tr` returns `renewable`, and the narration id is `erneuerbar`, so both scenes raised `KeyError` on attempts 2 and 3. The calls now pass `erneuerbar`. The retry is queued in `render-bim.ps1` ahead of cooling and the BIM lessons.
