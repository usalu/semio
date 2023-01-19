using System;
using System.Collections.Generic;
using System.Drawing;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class DeconstructDesignComponent : DeconstructComponent
    {
        public DeconstructDesignComponent()
          : base("Deconstruct Design", "DeDesign", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
        }
        protected override Bitmap Icon => Resources.icon_deconstruct_design;
        public override Guid ComponentGuid=>new ("452510D6-9554-4106-B740-363341BEDD0C");
    }
}