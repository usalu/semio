using System;
using System.Drawing;
using Grasshopper.Kernel;
using Semio.Grasshopper.Goos;
using Semio.Grasshopper.Params;

namespace Semio.Grasshopper.Components;

public abstract class SemioComponent : GH_Component
{
    public SemioComponent(string name, string nickname, string description, string category, string subcategory) : base(
        name, nickname, description, category, subcategory)
    {
    }
}