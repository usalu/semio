REPLACEMENTS = [
    ('        keep = {id(self._title), id(self._caption), id(self.source)}\n', '        keep = {id(self._caption), id(self.source)}\n'),
    ('        """🧹 Fade everything except title, caption and the model source tag."""', '        """🧹 Fade everything except the caption and the model source tag."""'),
    ('        keep = {id(self.big), id(self._title), id(self._caption), id(self.source)}\n', '        keep = {id(self.big), id(self._caption), id(self.source)}\n'),
]
