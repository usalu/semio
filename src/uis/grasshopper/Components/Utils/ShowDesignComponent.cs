using System;
using System.Collections.Generic;
using System.Linq;
using Grasshopper.Kernel;
using Rhino.FileIO;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Utils
{
    public class ShowDesignComponent : GH_Component
    {
        public ShowDesignComponent()
          : base("Show Design", ":Design", "", "Semio", "Utils")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new RepresentationParam());
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddGenericParameter("Objects", "O", "Imported rhino objects", GH_ParamAccess.list);
        }
        // TODO Update import logic to be in sync with constants and different file types.
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            //RepresentationGoo representation = new();
            //if (!DA.GetData(0, ref representation)) return;
            //File3dm file;
            //switch (representation.Value.Encoding)
            //{
            //    case Representation.Encoding.ByteArray:
            //        file = File3dm.FromByteArray(representation.Value.ByteArray.ToByteArray());
            //        break;
            //    case Representation.Encoding.Text:
            //    default:
            //        // Dummy
            //        file = new File3dm();
            //        break;
            //}
            
            //DA.SetDataList(0, file.Objects.Select(o=>o.Geometry));
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_show_design;

        public override Guid ComponentGuid => new ("464CD7FB-6994-4C81-B510-3EA5508CB650");

    }
}