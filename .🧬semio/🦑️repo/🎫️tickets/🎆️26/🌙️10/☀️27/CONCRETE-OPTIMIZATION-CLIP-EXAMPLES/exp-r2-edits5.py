REPLACEMENTS = [
    ("({beam} {self.tr('beam', 'Strahl')} + {sky} {self.tr('sky', 'Himmel')})", "({beam} {self.tr('direct', 'direkt')} + {sky} {self.tr('diffuse', 'diffus')})"),
    ("{self.tr('Beam stopped', 'Strahl gestoppt')}: {rays}", "{self.tr('Direct sun stopped', 'Direktsonne gestoppt')}: {rays}"),
    ("{self.tr('beam stopped', 'Strahl gestoppt')} {self.number(stopped[0])}", "{self.tr('direct sun stopped', 'Direktsonne gestoppt')} {self.number(stopped[0])}"),
]
