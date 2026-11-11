"""🎞️ Ticket-only live moving-family refresh experiment."""

class RefreshMoving:
    """🔄 Select current moving objects without repainting the saved static family."""

    def update_to_time(self, time):
        """⏱️ Apply frame updates before selecting the current moving objects."""
        super().update_to_time(time)
        if self.animations is not None:
            family = self.get_mobject_family_members()
            if getattr(self.renderer, "static_image", None) is None:
                self.moving_mobjects = family
            else:
                static = set(self.static_mobjects)
                self.moving_mobjects = [item for item in family if item.has_points() and item not in static]
                if not self.moving_mobjects:
                    self.renderer.static_image = None
                    self.moving_mobjects = family
