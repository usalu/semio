"""Import the four clips whose dt accumulators were replaced by canonical-clock timers."""
import importlib

for name in (
    "🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧲️fundamentals.🐍️",
    "🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧱️conduction.🐍️",
    "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔗️consistency.🐍️",
    "🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.⚖️regulations.🐍️",
    "🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.🧪️practice.🐍️",
):
    module = importlib.import_module(name)
    print("ok", name, hasattr(module, "_clock"))
