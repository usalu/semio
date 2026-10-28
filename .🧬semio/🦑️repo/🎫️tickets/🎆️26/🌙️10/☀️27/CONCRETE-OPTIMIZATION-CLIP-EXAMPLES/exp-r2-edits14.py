REPLACEMENTS = [('''            level = grow.get_value()
            value = value_fn() * level
''', '''            level = grow.get_value()
            if level <= .02:
                return _mn.VGroup(_mn.Dot(base, radius=1e-4, fill_opacity=0, stroke_width=0))
            value = value_fn() * level
''')]
