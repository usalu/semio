using System;
using System.Collections.Generic;

using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class ImportRepresentationComponent : DeconstructComponent
    {
        public ImportRepresentationComponent()
          : base("Deconstruct Representation", "DeRepresentation", "Deconstruct an element", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new RepresentationParam());
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddTextParameter("Body", "B", "", GH_ParamAccess.item);
            pManager.AddParameter(new EncodingParam());

            pManager.AddTextParameter("Type", "T", "", GH_ParamAccess.item);
            pManager.AddTextParameter("Name", "N", "", GH_ParamAccess.item);
            pManager.AddNumberParameter("Level of Detail", "LD", "", GH_ParamAccess.item);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            RepresentationGoo representation = new();
            if (!DA.GetData(0, ref representation)) return;
            byte[] body;
            
            DA.SetData(0, representation.Value);
            DA.SetData(1, representation.Value.Name);
            DA.SetData(2, representation.Value.Lod);
            DA.SetData(3, body);
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_deconstruct_representation;

        public override Guid ComponentGuid => new ("4E7849BC-25BB-4452-BFC5-B9E26E445AD1");

    }
}