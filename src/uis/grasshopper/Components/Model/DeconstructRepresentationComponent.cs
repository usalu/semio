using System;
using System.Collections.Generic;

using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;
using Semio.UI.Grasshopper.Utility;

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
            pManager.AddParameter(new FileTypeParam());
            pManager.AddParameter(new PlatformParam());
            pManager.AddTextParameter("Description", "Dc", "", GH_ParamAccess.item);
            pManager.AddTextParameter("Concepts", "Cp", "", GH_ParamAccess.list);
            pManager.AddNumberParameter("Level of Detail", "LD", "", GH_ParamAccess.item);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            RepresentationGoo representation = new();
            if (!DA.GetData(0, ref representation)) return;

            DA.SetData(0, Converter.ToString(representation.Value));
            DA.SetData(1, representation.Value.FileType);
            DA.SetData(2, representation.Value.Platform);
            DA.SetData(3, representation.Value.Description);
            DA.SetDataList(4, representation.Value.Concepts);
            DA.SetData(5, representation.Value.Lod);
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_deconstruct_representation;

        public override Guid ComponentGuid => new ("4E7849BC-25BB-4452-BFC5-B9E26E445AD1");

    }
}